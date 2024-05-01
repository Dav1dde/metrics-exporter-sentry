use std::{
    collections::HashMap,
    sync::{Arc, PoisonError, RwLock},
};

use sentry::metrics as sm;

type Units = Arc<RwLock<HashMap<metrics::KeyName, sm::MetricUnit>>>;

macro_rules! insert_unit {
    ($units:expr, $key:expr, $unit:expr) => {
        if let Some(unit) = $unit {
            let mut store = $units.write().unwrap_or_else(PoisonError::into_inner);
            store.insert($key, convert_unit(unit));
        }
    };
}

/// A Sentry recorder for metrics.
///
/// Exports all collected metrics to Sentry Metrics.
pub struct SentryRecorder {
    counters: Units,
    gauges: Units,
    histograms: Units,
}

impl SentryRecorder {
    /// Creates a new [`SentryRecorder`] emitting to metrics to [`sentry::Hub::current()`].
    ///
    /// **Note:** There is no `install` or similar convenience method to install the recorder,
    /// it needs to be explicitly set using [`metrics::set_global_recorder`] to prevent
    /// dependency version mismatches.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_exporter_sentry::SentryRecorder;
    ///
    /// metrics::set_global_recorder(SentryRecorder::new()).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            counters: Default::default(),
            gauges: Default::default(),
            histograms: Default::default(),
        }
    }
}

impl Default for SentryRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl metrics::Recorder for SentryRecorder {
    fn describe_counter(
        &self,
        key: metrics::KeyName,
        unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        insert_unit!(self.counters, key, unit);
    }

    fn describe_gauge(
        &self,
        key: metrics::KeyName,
        unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        insert_unit!(self.gauges, key, unit);
    }

    fn describe_histogram(
        &self,
        key: metrics::KeyName,
        unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        insert_unit!(self.histograms, key, unit);
    }

    fn register_counter(
        &self,
        key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Counter {
        metrics::Counter::from_arc(Handle::new(key, &self.counters))
    }

    fn register_gauge(
        &self,
        key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Gauge {
        metrics::Gauge::from_arc(Handle::new(key, &self.gauges))
    }

    fn register_histogram(
        &self,
        key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Histogram {
        metrics::Histogram::from_arc(Handle::new(key, &self.histograms))
    }
}

fn convert_unit(unit: metrics::Unit) -> sm::MetricUnit {
    use metrics::Unit;
    use sm::{DurationUnit::*, FractionUnit::*, InformationUnit::*, MetricUnit::*};
    match unit {
        Unit::Percent => Fraction(Percent),
        Unit::Seconds => Duration(Second),
        Unit::Milliseconds => Duration(MilliSecond),
        Unit::Microseconds => Duration(MicroSecond),
        Unit::Nanoseconds => Duration(NanoSecond),
        Unit::Tebibytes => Information(TebiByte),
        Unit::Gigibytes => Information(GibiByte),
        Unit::Mebibytes => Information(MebiByte),
        Unit::Kibibytes => Information(KibiByte),
        Unit::Bytes => Information(Byte),
        other => Custom(other.as_str().into()),
    }
}

struct Handle {
    key: metrics::Key,
    unit: Option<sm::MetricUnit>,
}

impl Handle {
    fn new(key: &metrics::Key, units: &Units) -> Arc<Handle> {
        let unit = units
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(key.name())
            .cloned();

        Arc::new(Handle {
            key: key.clone(),
            unit,
        })
    }

    fn apply_tags_and_unit(&self, mb: sm::MetricBuilder) -> sm::MetricBuilder {
        let mut unit = None;

        // TODO: Optimize conversions after: https://github.com/metrics-rs/metrics/pull/478
        let mb = self.key.labels().fold(mb, |mb, label| match label.key() {
            // Unit override.
            "unit" => {
                unit = label.value().parse().ok();
                mb
            }
            // Just another label.
            key => mb.with_tag(key.to_owned(), label.value().to_owned()),
        });

        let unit = unit
            .or_else(|| self.unit.clone())
            .unwrap_or(sm::MetricUnit::None);

        mb.with_unit(unit)
    }
}

impl metrics::CounterFn for Handle {
    fn increment(&self, value: u64) {
        let metric = sm::Metric::incr(self.key.name().to_owned(), value as f64);
        self.apply_tags_and_unit(metric).send();
    }

    fn absolute(&self, _value: u64) {
        // Not supported.
    }
}

impl metrics::GaugeFn for Handle {
    fn increment(&self, _value: f64) {
        // Not supported.
    }

    fn decrement(&self, _value: f64) {
        // Not supported.
    }

    fn set(&self, value: f64) {
        let metric = sm::Metric::gauge(self.key.name().to_owned(), value);
        self.apply_tags_and_unit(metric).send();
    }
}

impl metrics::HistogramFn for Handle {
    fn record(&self, value: f64) {
        let metric = sm::Metric::distribution(self.key.name().to_owned(), value);
        self.apply_tags_and_unit(metric).send();
    }
}

//! Metrics Exporter for Sentry.
//!
//! `metrics-exporter-sentry` is a [`metrics`] compatible exporter which
//! supports reporting metrics to [Sentry](https://docs.sentry.io/product/metrics/).
//!
//! # Usage
//!
//! ```no_run
//! use metrics_exporter_sentry::SentryRecorder;
//!
//! // Setup Sentry SDK.
//! let _sentry = sentry::init("https://key@sentry.io/123");
//!
//! // Install the exporter.
//! metrics::set_global_recorder(SentryRecorder::new()).unwrap();
//!
//! // Metric visible in Sentry!
//! metrics::counter!("requests").increment(1);
//! ```
//!
//! ## Units
//!
//! In Sentry units are an integral part of each metric, in fact you can emit
//! multiple metrics with the same name but a different unit and they are
//! treated as separate metrics!
//!
//! By default all metrics are emitted with the [`none`](sentry::metrics::MetricUnit::None) unit.
//! You can change the unit using metrics `describe_*` macros:
//!
//! ```
//! use metrics::{histogram, describe_histogram, Unit};
//! # let request_duration = std::time::Duration::from_secs(1);
//!
//! describe_histogram!("request.duration", Unit::Seconds, "Duration of a request in seconds");
//! histogram!("request.duration").record(request_duration);
//! ```
//!
//! Or by using the special `unit` tag:
//!
//! ```
//! use metrics::histogram;
//! # let request_duration = std::time::Duration::from_secs(1);
//!
//! histogram!("request.duration", "unit" => "seconds").record(request_duration);
//! ```
//!
//! The conversion from [`metrics::Unit`] to [`sentry::metrics::MetricUnit`] is on a best effort
//! basis and some variants of [`metrics::Unit`] are emitted as a
//! [custom](sentry::metrics::MetricUnit::Custom) Sentry unit.
//!
//! When passing the unit as a tag, the unit is parsed directly as a [`sentry::metrics::MetricUnit`],
//! allowing you to pass units which cannot be represented as a [`metrics::Unit`].
//!
//! Generally it's recommended to use the `describe_*` macros.
//!
//! # Incompatibilities
//!
//! [`metrics`] supports operations on counters and gauges which cannot be represented in Sentry
//! metrics.
//!
//! The [`Counter::absolute`](metrics::Counter::absolute), [`Gauge::increment`](metrics::Gauge::increment)
//! and [`Gauge::decrement`](metrics::Gauge::decrement) operations are ignored by the [`SentryRecorder`].
//!
//! Sentry supports another type of metrics, sets. At this time [`metrics`] does not support
//! sets and thus cannot be emitted with this exporter.

mod recorder;

pub use self::recorder::*;

metrics-exporter-sentry
=======================

[![Crates.io][crates-badge]][crates-url]
[![License][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![docs.rs][docsrs-badge]][docsrs-url]

[crates-badge]: https://img.shields.io/crates/v/metrics-exporter-sentry.svg
[crates-url]: https://crates.io/crates/metrics-exporter-sentry
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/Dav1dde/metrics-exporter-sentry/blob/master/LICENSE
[actions-badge]: https://github.com/Dav1dde/metrics-exporter-sentry/workflows/CI/badge.svg
[actions-url]: https://github.com/Dav1dde/metrics-exporter-sentry/actions?query=workflow%3ACI+branch%3Amaster
[docsrs-badge]: https://img.shields.io/docsrs/metrics-exporter-sentry
[docsrs-url]: https://docs.rs/metrics-exporter-sentry

A [metrics-rs](https://docs.rs/metrics/) exporter for Sentry Metrics. 
The exporter uses the [Sentry Rust SDK](https://docs.rs/sentry/) to emit the metrics.

## Example

```rs
use metrics::counter;

fn main() {
    let _sentry = sentry::init("https://public@example.com/123");

    metrics::set_global_recorder(metrics_exporter_sentry::SentryRecorder::new()).unwrap();

    counter!("hello_world", "color" => "green").increment(1);
}
```

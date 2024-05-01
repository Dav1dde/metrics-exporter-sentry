#![allow(missing_docs)]

use metrics::counter;

fn main() {
    let dsn = option_env!("SENTRY_DSN").unwrap_or("https://public@example.com/123");

    println!("Using Sentry DSN: {dsn}");
    let _sentry = sentry::init(dsn);

    metrics::set_global_recorder(metrics_exporter_sentry::SentryRecorder::new()).unwrap();

    counter!("example.run", "example" => "simple", "unit" => "exec").increment(1);
}

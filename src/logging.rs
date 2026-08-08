use std::io::stderr;
use std::io::IsTerminal;

use clap::ValueEnum;
use tracing_glog::Glog;
use tracing_glog::GlogFields;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
#[cfg(feature = "otlp")]
use tracing_subscriber::EnvFilter;

// This enum can be used to add `log-level` option to CLI binaries.
#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum LogLevels {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevels> for LevelFilter {
    fn from(public_level: LogLevels) -> Self {
        match public_level {
            LogLevels::Error => LevelFilter::ERROR,
            LogLevels::Warn => LevelFilter::WARN,
            LogLevels::Info => LevelFilter::INFO,
            LogLevels::Debug => LevelFilter::DEBUG,
            LogLevels::Trace => LevelFilter::TRACE,
        }
    }
}

/// Installs the process-wide `tracing` subscriber: glog-style output on
/// stderr, filtered by `-l/--log-level` as before. When built with the
/// `otlp` feature and `OTLP_ENDPOINT` is set, every span monitord/
/// monitord-exporter build via `tracing` (including the per-unit
/// `#[tracing::instrument]` spans in the `monitord` crate) is additionally
/// exported over OTLP/gRPC — see `otlp_tracer_provider` for the
/// non-blocking/failure-tolerant details. Returns the `SdkTracerProvider`
/// in that case so `main.rs` can flush it on graceful shutdown.
#[cfg(feature = "otlp")]
pub fn init(
    log_filter_level: LevelFilter,
    systemd_version: Option<String>,
    dbus_version: Option<String>,
) -> Option<opentelemetry_sdk::trace::SdkTracerProvider> {
    use opentelemetry::trace::TracerProvider as _;

    let fmt = fmt::Layer::default()
        .with_writer(std::io::stderr)
        .with_ansi(stderr().is_terminal())
        .event_format(Glog::default().with_timer(tracing_glog::LocalTime::default()))
        .fmt_fields(GlogFields::default())
        .with_filter(log_filter_level);

    let tracer_provider = otlp_tracer_provider(systemd_version, dbus_version);
    // Independent filter from the console layer above: enabling OTLP export
    // shouldn't force -l debug (and therefore noisier console/journal
    // output) just to get spans worth looking at in Tempo.
    let otel_layer = tracer_provider.clone().map(|provider| {
        tracing_opentelemetry::layer()
            .with_tracer(provider.tracer(env!("CARGO_PKG_NAME")))
            .with_filter(EnvFilter::new("debug"))
    });

    tracing_subscriber::registry()
        .with(fmt)
        .with(otel_layer)
        .init();

    tracing::debug!("Logging is setup with {:?} level", log_filter_level);
    tracer_provider
}

#[cfg(not(feature = "otlp"))]
pub fn init(
    log_filter_level: LevelFilter,
    _systemd_version: Option<String>,
    _dbus_version: Option<String>,
) {
    let fmt = fmt::Layer::default()
        .with_writer(std::io::stderr)
        .with_ansi(stderr().is_terminal())
        .event_format(Glog::default().with_timer(tracing_glog::LocalTime::default()))
        .fmt_fields(GlogFields::default())
        .with_filter(log_filter_level);

    tracing_subscriber::registry().with(fmt).init();
    tracing::debug!("Logging is setup with {:?} level", log_filter_level);
}

/// Builds an OTLP-bound tracer provider from `OTLP_ENDPOINT`, or `None` if
/// it's unset (checked here, not threaded through `Config`, so this module
/// stays self-contained). Mirrors mcp_info_server's `logging.rs` exactly —
/// same fleet, same Tempo instance, same failure-tolerant design:
///
/// - Spans are handed off to an in-memory bounded queue and exported by a
///   dedicated background task (`opentelemetry_sdk::runtime::Tokio`) on its
///   own schedule, never inline with collection. A full queue just drops
///   the oldest span rather than blocking a producer, per the SDK's
///   standard behavior — this matters here specifically because collection
///   sits directly on the Prometheus scrape-latency critical path.
/// - Both the TCP connect and the RPC call are bounded by a 5s timeout —
///   `opentelemetry-otlp`'s own `with_endpoint()`/`with_timeout()` only
///   ever set the RPC-call timeout, never `connect_timeout()`, so a true
///   network black hole would otherwise hang on tonic/hyper's default
///   (~30s) before the RPC-level timeout even applies. Building the
///   `Channel` by hand is the only way to set both.
/// - A malformed endpoint or exporter construction failure is logged once,
///   here, and just disables OTLP export for this process rather than
///   failing startup — it uses `eprintln!`, not `tracing::warn!`, because
///   this runs before the `tracing` subscriber this function is building
///   is installed.
#[cfg(feature = "otlp")]
fn otlp_tracer_provider(
    systemd_version: Option<String>,
    dbus_version: Option<String>,
) -> Option<opentelemetry_sdk::trace::SdkTracerProvider> {
    use std::time::Duration;

    use opentelemetry::KeyValue;
    use opentelemetry_otlp::WithTonicConfig;
    use opentelemetry_sdk::runtime;
    use opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use opentelemetry_sdk::Resource;

    const OTLP_EXPORT_TIMEOUT: Duration = Duration::from_secs(5);

    let endpoint = std::env::var("OTLP_ENDPOINT").ok()?;

    let channel = match tonic::transport::Endpoint::from_shared(endpoint) {
        Ok(endpoint) => endpoint
            .connect_timeout(OTLP_EXPORT_TIMEOUT)
            .timeout(OTLP_EXPORT_TIMEOUT)
            .connect_lazy(),
        Err(e) => {
            eprintln!("OTLP_ENDPOINT is not a valid URI, disabling OTLP export: {e}");
            return None;
        }
    };

    let exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_channel(channel)
        .build()
    {
        Ok(exporter) => exporter,
        Err(e) => {
            eprintln!("failed to build OTLP exporter, disabling OTLP export: {e}");
            return None;
        }
    };

    // Doesn't need to match Prometheus's `instance` labels or any other
    // naming convention -- just needs to be unique/identifying enough to
    // tell hosts apart in Tempo, so the raw kernel hostname (whatever
    // format a given host happens to have it in) is fine as-is.
    let host_name = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let mut resource_builder = Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")))
        .with_attribute(KeyValue::new("host.name", host_name));
    if let Some(v) = systemd_version {
        resource_builder = resource_builder.with_attribute(KeyValue::new("systemd.version", v));
    }
    if let Some(v) = dbus_version {
        resource_builder = resource_builder.with_attribute(KeyValue::new("dbus.version", v));
    }
    let resource = resource_builder.build();

    let batch = BatchSpanProcessor::builder(exporter, runtime::Tokio).build();

    Some(
        SdkTracerProvider::builder()
            .with_resource(resource)
            .with_span_processor(batch)
            .build(),
    )
}

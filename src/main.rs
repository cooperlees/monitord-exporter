use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use anyhow::Result;
use clap::Parser;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::error;
use tracing::info;

const LONG_ABOUT: &str = "prometheus exporter to share how happy your systemd is ! 😊";

const CONFIG_CONFLICTS: &[&str] = &[
    "dbus_address",
    "no_networkd",
    "no_pid1",
    "no_system_state",
    "networkd_state_file_path",
    "services",
    "no_timers",
    "no_dbus",
    "no_unit_states",
    "no_machines",
    "timers",
    "boot_blame",
    "boot_blame_count",
    "no_boot_cache",
    "verify",
    "per_unit_concurrency",
];

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Path to a monitord.conf config file. Mutually exclusive with all other config arguments.
    /// Supported since monitord-exporter >= 0.19.0.
    #[clap(
        short = 'c',
        long,
        value_parser,
        conflicts_with_all = CONFIG_CONFLICTS
    )]
    config: Option<PathBuf>,
    /// dbus address
    #[clap(
        short,
        long,
        value_parser,
        default_value = "unix:path=/run/dbus/system_bus_socket",
        conflicts_with = "config"
    )]
    dbus_address: String,
    /// Adjust the console log-level
    #[arg(long, short, value_enum, ignore_case = true, default_value = "Info")]
    log_level: monitord::logging::LogLevels,
    /// networkd stats disable
    #[clap(long, conflicts_with = "config")]
    no_networkd: bool,
    /// pid1 stats disable
    #[clap(long, conflicts_with = "config")]
    no_pid1: bool,
    /// system state stats disable
    #[clap(long, conflicts_with = "config")]
    no_system_state: bool,
    /// network netif dir
    #[clap(
        long,
        value_parser,
        default_value = "/run/systemd/netif/links",
        conflicts_with = "config"
    )]
    networkd_state_file_path: PathBuf,
    /// TCP Port to listen on
    #[clap(short, long, value_parser, default_value_t = 1)]
    port: u16,
    /// Services to get service stats for
    #[clap(short, long, conflicts_with = "config")]
    services: Vec<String>,
    /// Disable timer stats
    #[clap(long, conflicts_with = "config")]
    no_timers: bool,
    /// Disable D-Bus stats
    #[clap(long, conflicts_with = "config")]
    no_dbus: bool,
    /// Disable per-unit state tracking
    #[clap(long, conflicts_with = "config")]
    no_unit_states: bool,
    /// Disable machine/container stats
    #[clap(long, conflicts_with = "config")]
    no_machines: bool,
    /// Specific timers to track
    #[clap(long, conflicts_with = "config")]
    timers: Vec<String>,
    /// Enable boot blame stats (slowest N units at boot)
    #[clap(long, conflicts_with = "config")]
    boot_blame: bool,
    /// Number of slowest boot blame units to report (requires --boot-blame)
    #[clap(
        long,
        value_parser,
        default_value_t = 5,
        requires = "boot_blame",
        conflicts_with = "config"
    )]
    boot_blame_count: u64,
    /// Disable boot blame result caching (requires --boot-blame)
    #[clap(long, requires = "boot_blame", conflicts_with = "config")]
    no_boot_cache: bool,
    /// Enable unit verification stats (systemd-analyze verify)
    #[clap(long, conflicts_with = "config")]
    verify: bool,
    /// Max units whose D-Bus work runs concurrently in the per-unit collection loop
    #[clap(long, value_parser, default_value_t = 8, conflicts_with = "config")]
    per_unit_concurrency: u64,
}

/// `dbus-daemon --version` prints a single line like "D-Bus Message Bus
/// Daemon 1.16.2" -- there's no D-Bus-protocol-level way to query the bus
/// daemon's own version (unlike systemd, which exposes one as a Manager
/// property), so this shells out instead. Best-effort: `None` on any
/// failure (binary not on PATH, unexpected output, etc.) rather than
/// blocking OTLP setup on it.
#[cfg(feature = "otlp")]
fn dbus_daemon_version() -> Option<String> {
    let output = std::process::Command::new("dbus-daemon")
        .arg("--version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()?
        .split_whitespace()
        .last()
        .map(str::to_string)
}

/// Signal handler to exit cleanly. Takes the tracer provider (a no-op `()`
/// when the `otlp` feature isn't compiled in) so a SIGINT flushes any
/// buffered OTLP spans before exiting rather than dropping them.
#[cfg(feature = "otlp")]
fn signal_handler(tracer_provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>) {
    let mut signals = match signal_hook::iterator::Signals::new([signal_hook::consts::SIGINT]) {
        Ok(sig) => sig,
        Err(err) => panic!("Unable to handle SIGINT: {:#?}", err),
    };
    for sig in signals.forever() {
        // TODO: Print signal name somehow ...
        info!("Received signal {:?} .. Exiting ...", sig);
        if sig == signal_hook::consts::SIGINT {
            if let Some(provider) = &tracer_provider {
                if let Err(e) = provider.shutdown() {
                    tracing::warn!("failed to flush OTLP tracer provider on shutdown: {e}");
                }
            }
            std::process::exit(0);
        }
    }
}

#[cfg(not(feature = "otlp"))]
fn signal_handler(_tracer_provider: ()) {
    let mut signals = match signal_hook::iterator::Signals::new([signal_hook::consts::SIGINT]) {
        Ok(sig) => sig,
        Err(err) => panic!("Unable to handle SIGINT: {:#?}", err),
    };
    for sig in signals.forever() {
        // TODO: Print signal name somehow ...
        info!("Received signal {:?} .. Exiting ...", sig);
        if sig == signal_hook::consts::SIGINT {
            std::process::exit(0);
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // main() is a plain sync fn (not #[tokio::main]) -- the runtime is
    // created here, up front, so logging::init()'s OTLP BatchSpanProcessor
    // (which spawns its background export task onto runtime::Tokio at
    // construction time) has an active Tokio context to spawn onto.
    // Without this, otlp builds panic immediately ("there is no reactor
    // running") since the runtime otherwise isn't created until later.
    // The enter() guard is scoped tightly to just the init() call and
    // dropped immediately after -- the spawned task keeps running on rt's
    // own worker threads regardless, and this avoids any risk of a stale
    // entered-context interacting with the request loop's rt.block_on()
    // calls below.
    let rt = Runtime::new().expect("Unable to get an async runtime");

    // Only bother fetching these (for OTLP resource attributes) when OTLP
    // export is actually going to be configured -- avoids a wasted D-Bus
    // round trip and subprocess spawn on every startup otherwise.
    // Deliberately done via rt.block_on() here, before rt.enter() below,
    // rather than inside logging::init() itself -- keeps this D-Bus/
    // subprocess work on the same already-proven-safe "plain block_on"
    // path the request loop uses, with no risk of nesting it under an
    // active enter() guard.
    #[cfg(feature = "otlp")]
    let (systemd_version, dbus_version) = if std::env::var("OTLP_ENDPOINT").is_ok() {
        let systemd_version = rt.block_on(async {
            let conn = zbus::Connection::system().await.ok()?;
            monitord::system::get_version(&conn).await.ok()
        });
        (
            systemd_version.map(|v| v.to_string()),
            dbus_daemon_version(),
        )
    } else {
        (None, None)
    };
    #[cfg(not(feature = "otlp"))]
    let (systemd_version, dbus_version) = (None, None);

    // `()` without the "otlp" feature -- kept as a binding (not discarded)
    // so signal_handler's uniform signature below works for both builds.
    #[allow(clippy::let_unit_value)]
    let tracer_provider = {
        let _rt_guard = rt.enter();
        monitord_exporter::logging::init(args.log_level.into(), systemd_version, dbus_version)
    };

    info!("Starting monitord-exporter on port {}", args.port);

    let bind_uri = format!("[::]:{}", args.port);
    let binding = bind_uri.parse().unwrap();
    let exporter = match prometheus_exporter::start(binding) {
        Ok(exp) => exp,
        Err(err) => {
            error!("Failed to start prometheus exporter: {:#?}", err);
            std::process::exit(1)
        }
    };

    thread::spawn(move || signal_handler(tracer_provider));

    let mut prom_metrics = monitord_exporter::metrics::MonitordPromStats::new();

    // TODO: See if we can supply services in the prometheus scrape as params
    // - This will probably need to move config parsing back into the request loop
    // Generate a monitord config struct from a config file or CLI arguments
    let monitord_config = if let Some(config_path) = &args.config {
        info!("Loading monitord config from {:?}", config_path);
        monitord_exporter::config::load_from_file(config_path)?
    } else {
        monitord_exporter::config::build_from_cli(
            &args.dbus_address,
            args.no_networkd,
            &args.networkd_state_file_path,
            args.no_pid1,
            args.no_system_state,
            &args.services,
            args.no_timers,
            &args.timers,
            args.no_dbus,
            args.no_unit_states,
            args.no_machines,
            args.boot_blame,
            args.boot_blame_count,
            args.no_boot_cache,
            args.verify,
            args.per_unit_concurrency,
        )
    };
    let mut cached_dbus_connection: Option<zbus::Connection> = None;
    loop {
        let guard = exporter.wait_request();
        let locked_monitord_stats = Arc::new(RwLock::new(monitord::MonitordStats::default()));
        cached_dbus_connection = rt
            .block_on(monitord::stat_collector(
                monitord_config.clone(),
                Some(locked_monitord_stats.clone()),
                false,
                cached_dbus_connection.take(),
            ))
            .map_err(anyhow::Error::from)?;
        let monitord_stats = rt.block_on(locked_monitord_stats.read());
        debug!("Stats collected: {:?}", monitord_stats);
        // Convert monitord stats into prometheus objects
        prom_metrics.populate(&monitord_config, &monitord_stats);
        drop(guard);
        info!("Stats refreshed and served");
    }
}

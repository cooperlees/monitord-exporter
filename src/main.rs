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
}

/// Signal handler to exit cleanly
fn signal_handler() {
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
    monitord_exporter::logging::setup_logging(args.log_level.into());

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

    thread::spawn(signal_handler);

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
        )
    };
    let rt = Runtime::new().expect("Unable to get an async runtime");
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

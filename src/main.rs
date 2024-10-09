use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::error;
use tracing::info;

const LONG_ABOUT: &str = "prometheus exporter to share how happy your systemd is ! 😊";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// dbus address
    #[clap(
        short,
        long,
        value_parser,
        default_value = "unix:path=/run/dbus/system_bus_socket"
    )]
    dbus_address: String,
    /// Adjust the console log-level
    #[arg(long, short, value_enum, ignore_case = true, default_value = "Info")]
    log_level: monitord::logging::LogLevels,
    /// networkd stats disable
    #[clap(long)]
    no_networkd: bool,
    /// pid1 stats disable
    #[clap(long)]
    no_pid1: bool,
    /// system state stats disable
    #[clap(long)]
    no_system_state: bool,
    /// network netif dir
    #[clap(long, value_parser, default_value = "/run/systemd/netif/links")]
    networkd_state_file_path: PathBuf,
    /// TCP Port to listen on
    #[clap(short, long, value_parser, default_value_t = 1)]
    port: u16,
    /// Services to get service stats for
    #[clap(short, long)]
    services: Vec<String>,
}

/// Signal handler to exit cleanly
async fn signal_handler() {
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

// Ignoring that we use a non async friendly lock
// That is on purpose here as we only want to support 1 request at a time
#[allow(clippy::await_holding_lock)]
#[tokio::main]
async fn main() -> Result<()> {
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

    tokio::task::spawn(signal_handler());

    let locked_monitord_stats = Arc::new(RwLock::new(monitord::MonitordStats::default()));
    let mut prom_metrics = monitord_exporter::metrics::MonitordPromStats::new();

    // TODO: See if we can supply services in the prometheus scrape as params
    // - This will probably need to move config parsing back into the request loop
    // Generate a monitord config struct from CLI arguments
    let mut monitord_config = monitord::config::Config::default();
    monitord_config.monitord.dbus_address = args.dbus_address.clone();
    monitord_config.networkd.enabled = !args.no_networkd;
    monitord_config.networkd.link_state_dir = args.networkd_state_file_path.clone();
    monitord_config.pid1.enabled = !args.no_pid1;
    monitord_config.system_state.enabled = !args.no_system_state;
    monitord_config.services.extend(args.services.clone());
    loop {
        let guard = exporter.wait_request();

        match monitord::stat_collector(
            monitord_config.clone(),
            Some(locked_monitord_stats.clone()),
            false,
        )
        .await
        {
            Ok(_) => {
                {
                    let monitord_stats = locked_monitord_stats.read().await;
                    debug!("Stats collected: {:?}", monitord_stats);
                    // Convert monitord stats into prometheus objects
                    prom_metrics.populate(&monitord_config, &monitord_stats);
                }
            }
            Err(err) => error!("Stats failed to collect: {:?}", err),
        }

        drop(guard);
        info!("Stats refreshed and served");
    }
}

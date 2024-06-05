use std::path::PathBuf;
use std::thread;

use anyhow::Result;
use clap::Parser;
use indexmap::IndexMap;
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
    /// network netif dir
    #[clap(short, long, value_parser, default_value = "/run/systemd/netif/links")]
    networkd_state_file_path: PathBuf,
    /// TCP Port to listen on
    #[clap(short, long, value_parser, default_value_t = 1)]
    port: u16,
    /// Services to get service stats for
    #[clap(short, long)]
    services: Vec<String>,
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

    let mut monitord_stats = monitord::MonitordStats::default();
    let mut prom_metrics = monitord_exporter::metrics::MonitordPromStats::new();
    loop {
        let guard = exporter.wait_request();
        // TODO: CLI to disable/enable networkd
        match monitord::networkd::parse_interface_state_files(
            args.networkd_state_file_path.clone(),
            None,
            &args.dbus_address,
        ) {
            Ok(networkd_stats) => monitord_stats.networkd = networkd_stats,
            Err(err) => error!("networkd stats failed: {}", err),
        }

        // Collect PID1 stats
        monitord_stats.pid1 = match monitord::pid1::get_pid1_stats() {
            Ok(p1s) => Some(p1s),
            Err(err) => {
                error!("Failed to get PID1 stats: {err:#?}");
                None
            }
        };

        // TODO: See if we can supply services in the prometheus scrape as params
        let mut monitord_config: IndexMap<String, IndexMap<String, Option<String>>> =
            IndexMap::new();
        monitord_config.insert(String::from("services"), IndexMap::new());
        monitord_config.insert(String::from("units"), IndexMap::new());
        for service in &args.services {
            monitord_config["services"].insert(service.clone(), None);
        }
        match monitord::units::parse_unit_state(&args.dbus_address, monitord_config) {
            Ok(units_stats) => monitord_stats.units = units_stats,
            Err(err) => error!("units stats failed: {}", err),
        }

        // Collect system state
        monitord_stats.system_state = match monitord::system::get_system_state(&args.dbus_address) {
            Ok(ss) => ss,
            Err(err) => {
                error!("Failed to get system state: {err:#?}");
                monitord::system::SystemdSystemState::unknown
            }
        };

        debug!("Stats collected: {:?}", monitord_stats);

        // Convert monitord stats into prometheus objects
        prom_metrics.populate(&monitord_stats);

        drop(guard);
        info!("Stats refreshed and served");
    }
}

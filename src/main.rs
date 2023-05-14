use std::path::PathBuf;
use std::thread;

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::debug;
use log::error;
use log::info;

mod metrics;

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
    /// network netif dir
    #[clap(short, long, value_parser, default_value = "/run/systemd/netif/links")]
    networkd_state_file_path: PathBuf,
    /// TCP Port to listen on
    #[clap(short, long, value_parser, default_value_t = 1)]
    port: u16,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
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
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

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
    let mut prom_metrics = crate::metrics::MonitordPromStats::new();
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
        // TOOD: Support service to pull stats on
        let services_to_get_stats_on = Vec::from([]);
        match monitord::units::parse_unit_state(&args.dbus_address, services_to_get_stats_on) {
            Ok(units_stats) => monitord_stats.units = units_stats,
            Err(err) => error!("units stats failed: {}", err),
        }
        debug!("Stats collected: {:?}", monitord_stats);

        // Convert monitord stats into prometheus objects
        prom_metrics.populate(&monitord_stats);

        drop(guard);
        info!("Stats refreshed and served");
    }
}

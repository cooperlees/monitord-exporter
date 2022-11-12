use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::info;

const LONG_ABOUT: &str = "prometheus export to share how happy your systemd is ! 😊";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Number that brings you luck
    #[clap(short, long, value_parser, default_value_t = 69)]
    lucky_number: u8,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("monitord-exporter coming soon!");

    Ok(())
}

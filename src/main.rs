mod config;

use std::env;
use std::path::PathBuf;
use log::LevelFilter;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    path: Option<PathBuf>,

    #[arg(long)]
    verbose: bool,

    #[arg(long)]
    trace: bool,
}

fn main() {
    let args = Args::parse();

    let log_level = if args.trace {
        LevelFilter::Trace
    } else if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .init();

    if log_level != LevelFilter::Info {
        log::info!("Setting log level to {}", log_level);
    }

    let working_dir = args.path.unwrap_or_else(|| env::current_dir().expect("failed to determine current directory"));

    let working_dir = working_dir
        .canonicalize()
        .unwrap_or_else(|_| working_dir.clone());

    // let gix = gix::open(&working_dir);
    log::info!("Working directory: {}", working_dir.display());

    if let Ok(config) = config::load_from_file(working_dir) {

    } else  {
        log::error!("Failed to load config file.");
        std::process::exit(1);
    }
}

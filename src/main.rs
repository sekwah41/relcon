mod config;
mod git;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use log::LevelFilter;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Project root must contain a relcon.json file and be a git folder
    /// Defaults to the current directory
    #[arg(long)]
    path: Option<PathBuf>,

    /// Print verbose logs
    #[arg(long)]
    verbose: bool,

    /// Print all logs
    #[arg(long)]
    trace: bool,

    ///
    #[arg()]
    release_as: String,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    log::info!("Working directory: {}", working_dir.display());

    if let Ok(config) = config::load_from_file(&working_dir) {

    } else  {
        log::error!("Failed to load config file.");
        std::process::exit(1);
    }
    // Could use discover(".") if we want recursive finding, but we should be running from the root
    let repo = gix::open(&working_dir).map_err(|e| {
        log::error!("Failed to open git directory {}", e);
        e
    })?;

    let tag = git::latest_tag(&repo)?;

    let commit_history = git::commit_history(&repo);

    Ok(())
}

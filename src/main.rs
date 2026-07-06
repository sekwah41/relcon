use std::env;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let working_dir = args.path.unwrap_or_else(|| env::current_dir().expect("failed to determine current directory"));

    // let gix = gix::open(&working_dir);
    println!("Working directory: {}", working_dir.display());
}

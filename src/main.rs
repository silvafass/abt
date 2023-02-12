mod fs;
mod settings;
mod tool;
mod types;

use clap::Parser;
use std::{path::PathBuf, time::Instant};

use crate::settings::Config;

/// That is another backup tool
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Turn debugging information on
    #[arg(long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    /// Create/Update config file
    Config {
        /// Profile name
        profile: String,
        /// Source directory
        #[arg(short, long)]
        source: Option<PathBuf>,
        /// Target directory
        #[arg(short, long)]
        target: Option<PathBuf>,
    },
    /// Copy files from the source directory to the backup directory
    Backup {
        /// Profile name
        #[arg(short, long)]
        profile: String,
    },
    /// Restore files from the backup directory to the source directory
    Restore,
}

fn main() -> types::BackupResult<()> {
    let start = Instant::now();
    let args = Args::parse();

    if args.debug {
        println!("Debug mode enabled.")
    }

    match args.action {
        Action::Config {
            profile,
            source,
            target,
        } => {
            let mut config = settings::Backup::load_or_create(&profile, &source, &target)
                .unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    std::process::exit(1)
                });
            if let Some(source) = source {
                config.source_path(&source);
            }
            if let Some(target) = target {
                config.target_path(&target);
            }
            config = config.save()?;
            println!("Loaded: {:#?}", config);
        }
        Action::Backup { profile } => {
            println!("Backup action.");
            tool::backup(&profile)?
        }
        _ => todo!(),
    }
    println!("Elapsed time of {:?}", start.elapsed());
    Ok(())
}

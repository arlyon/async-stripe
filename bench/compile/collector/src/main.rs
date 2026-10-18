mod collect;
mod metrics;
mod model;
mod process;
mod report;

use std::num::NonZeroUsize;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Measure library-consumer build sizes; optionally collect noisy diagnostics")]
struct Cli {
    #[arg(long, global = true, help = "Log full commands as well as progress")]
    verbose: bool,
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    Collect(Collect),
    /// Compare two report directories or results.json files.
    Compare {
        baseline: PathBuf,
        current: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, help = "Show deltas even when toolchains or build settings differ")]
        allow_mismatch: bool,
    },
}

#[derive(Args)]
struct Collect {
    #[arg(long, default_value = "minimal,requests", help = "Comma-separated workloads, or all")]
    cases: String,
    #[arg(long, value_delimiter = ',', default_value = "dev,release")]
    profiles: Vec<String>,
    #[arg(
        long,
        help = "New report directory, relative to --repo (default: fresh directory under target/compile-benchmarks)"
    )]
    output: Option<PathBuf>,
    #[arg(long, help = "Total clean builds per case/profile, including the size build")]
    timings: Option<NonZeroUsize>,
    #[arg(long, help = "Collect consumer-root and standalone shared-library LLVM IR totals")]
    llvm_lines: bool,
    #[arg(long, help = "Forbid dependency downloads")]
    offline: bool,
    #[arg(long, help = "Compilation target (defaults to the rustc host)")]
    target: Option<String>,
    #[arg(long, help = "Repository to measure (defaults to this checkout)")]
    repo: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_target(false)
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_max_level(if cli.verbose { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .init();
    match cli.command {
        Action::Collect(args) => collect::collect(args),
        Action::Compare { baseline, current, output, allow_mismatch } => {
            let markdown = report::compare_files(&baseline, &current, allow_mismatch)?;
            if let Some(path) = output {
                std::fs::write(&path, markdown)?;
                tracing::info!("Wrote {}", path.display());
            } else {
                print!("{markdown}");
            }
            Ok(())
        }
    }
}

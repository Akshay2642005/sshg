mod config;
mod ssh;
mod ui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sshman")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch TUI
    Tui,
    /// Add a new SSH host
    Add,
    /// Connect to a host
    Connect {
        host: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Tui) | None => ui::start(),
        Some(Commands::Add) => config::add_new_host(),
        Some(Commands::Connect { host }) => ssh::connect(host),
    }
}


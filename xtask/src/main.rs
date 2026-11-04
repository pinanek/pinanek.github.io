mod query;

use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(bin_name = "cargo xtask")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Manage Tree-sitter queries.
    Query {
        #[command(subcommand)]
        command: QueryCommand,
    },
}

#[derive(Subcommand)]
enum QueryCommand {
    /// Pull queries from pinned upstream revisions.
    Pull,
    /// Update upstream revision pins.
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the workspace");

    match cli.command {
        Command::Query { command } => match command {
            QueryCommand::Pull => query::pull(root),
            QueryCommand::Update => query::update(root),
        },
    }
}

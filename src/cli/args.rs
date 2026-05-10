use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "bluebond")]
#[command(about = "Fix dual-boot Bluetooth pairing by syncing Windows bond keys into BlueZ.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check host tools and paths needed by BlueBond.
    Doctor,
}

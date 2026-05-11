use std::path::PathBuf;

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
    /// Preview BlueZ apply changes without making changes.
    Apply {
        /// Only preview changes; do not write BlueZ files or restart Bluetooth.
        #[arg(long)]
        dry_run: bool,

        /// Override the BlueZ store directory.
        #[arg(long, value_name = "PATH")]
        bluez_dir: Option<PathBuf>,

        /// Override the offline Windows root directory.
        #[arg(long, value_name = "PATH")]
        windows_root: Option<PathBuf>,
    },

    /// Check host tools and paths needed by BlueBond.
    Doctor,

    /// Generate a dry-run BlueZ sync plan without making changes.
    Plan {
        /// Override the BlueZ store directory.
        #[arg(long, value_name = "PATH")]
        bluez_dir: Option<PathBuf>,

        /// Override the offline Windows root directory.
        #[arg(long, value_name = "PATH")]
        windows_root: Option<PathBuf>,

        /// Print the dry-run plan as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Read Linux BlueZ adapters and devices without making changes.
    Scan {
        /// Override the BlueZ store directory.
        #[arg(long, value_name = "PATH")]
        bluez_dir: Option<PathBuf>,

        /// Override the offline Windows root directory.
        #[arg(long, value_name = "PATH")]
        windows_root: Option<PathBuf>,
    },
}

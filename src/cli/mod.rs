mod args;
mod output;

use std::process::ExitCode;

use clap::Parser;

use crate::app;
use args::{Cli, Command};

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Doctor => {
            let report = app::doctor::run();
            output::print_doctor_report(&report);

            if report.has_failures() {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Command::Scan {
            bluez_dir,
            windows_root,
        } => {
            let request = match bluez_dir {
                Some(bluez_dir) => app::scan::ScanRequest::new(bluez_dir),
                None => app::scan::default_request(),
            };

            let request = match windows_root {
                Some(windows_root) => request.with_windows_root(windows_root),
                None => request,
            };

            match app::scan::run(&request) {
                Ok(report) => {
                    output::print_scan_report(&report);
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("bluebond scan failed: {error}");
                    ExitCode::from(1)
                }
            }
        }
    }
}

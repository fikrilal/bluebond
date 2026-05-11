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
        Command::Plan {
            bluez_dir,
            windows_root,
            json,
        } => {
            let scan_request = build_scan_request(bluez_dir.clone(), windows_root);

            match app::scan::run(&scan_request) {
                Ok(scan_report) => {
                    let render_request = match bluez_dir {
                        Some(bluez_dir) => app::plan::RenderPlanRequest::new(bluez_dir),
                        None => app::plan::default_request(),
                    };
                    let plan_report = app::plan::build_plan(&scan_report, &render_request);

                    match output::print_plan_report(&plan_report, json) {
                        Ok(()) => ExitCode::SUCCESS,
                        Err(error) => {
                            eprintln!("bluebond plan failed: {error}");
                            ExitCode::from(1)
                        }
                    }
                }
                Err(error) => {
                    eprintln!("bluebond plan failed: {error}");
                    ExitCode::from(1)
                }
            }
        }
        Command::Scan {
            bluez_dir,
            windows_root,
        } => {
            let request = build_scan_request(bluez_dir, windows_root);

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

fn build_scan_request(
    bluez_dir: Option<std::path::PathBuf>,
    windows_root: Option<std::path::PathBuf>,
) -> app::scan::ScanRequest {
    let request = match bluez_dir {
        Some(bluez_dir) => app::scan::ScanRequest::new(bluez_dir),
        None => app::scan::default_request(),
    };

    match windows_root {
        Some(windows_root) => request.with_windows_root(windows_root),
        None => request,
    }
}

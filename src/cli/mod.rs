mod args;
mod output;

use std::process::ExitCode;

use clap::Parser;

use crate::app;
use args::{Cli, Command, RollbackCommand};

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Apply {
            dry_run,
            execute,
            bluez_dir,
            windows_root,
            adapter,
            target_device,
            windows_source_device,
        } => {
            if dry_run == execute {
                eprintln!("bluebond apply requires exactly one of --dry-run or --execute");
                return ExitCode::from(2);
            }
            let manual_selection =
                match build_manual_selection(adapter, target_device, windows_source_device) {
                    Ok(selection) => selection,
                    Err(message) => {
                        eprintln!("{message}");
                        return ExitCode::from(2);
                    }
                };

            let scan_request = build_scan_request(bluez_dir, windows_root);

            if dry_run {
                let apply_request = match manual_selection {
                    Some(selection) => {
                        app::apply::default_dry_run_request().with_manual_selection(selection)
                    }
                    None => app::apply::default_dry_run_request(),
                };

                match app::scan::run(&scan_request).and_then(|scan_report| {
                    app::apply::build_dry_run_report(&scan_report, &apply_request)
                }) {
                    Ok(report) => {
                        output::print_apply_dry_run_report(&report);
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("bluebond apply dry-run failed: {error}");
                        ExitCode::from(1)
                    }
                }
            } else {
                let apply_request = match manual_selection {
                    Some(selection) => {
                        app::apply::default_execute_request().with_manual_selection(selection)
                    }
                    None => app::apply::default_execute_request(),
                };

                match app::scan::run(&scan_request)
                    .and_then(|scan_report| app::apply::execute_apply(&scan_report, &apply_request))
                {
                    Ok(report) => {
                        output::print_apply_execute_report(&report);
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("bluebond apply execute failed: {error}");
                        ExitCode::from(1)
                    }
                }
            }
        }
        Command::Doctor => {
            let report = app::doctor::run();
            output::print_doctor_report(&report);

            if report.has_failures() {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Command::Rollback { command } => match command {
            RollbackCommand::List { backup_dir } => {
                let backup_dir = backup_dir.unwrap_or_else(app::rollback::default_backup_dir);

                match app::rollback::list_backups(&backup_dir) {
                    Ok(report) => {
                        output::print_rollback_backup_list(&report);
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("bluebond rollback list failed: {error}");
                        ExitCode::from(1)
                    }
                }
            }
            RollbackCommand::Restore { metadata } => {
                let request = app::rollback::RollbackRestoreRequest {
                    metadata_path: metadata,
                };

                match app::rollback::restore_backup(&request) {
                    Ok(report) => {
                        output::print_rollback_restore_report(&report);
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("bluebond rollback restore failed: {error}");
                        ExitCode::from(1)
                    }
                }
            }
        },
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

fn build_manual_selection(
    adapter: Option<String>,
    target_device: Option<String>,
    windows_source_device: Option<String>,
) -> std::result::Result<Option<app::apply::ManualApplySelection>, String> {
    match (target_device.as_deref(), windows_source_device.as_deref()) {
        (None, None) => {
            if adapter.is_some() {
                Err("bluebond apply --adapter requires --target-device and --windows-source-device"
                    .to_string())
            } else {
                Ok(None)
            }
        }
        (Some(target_device), Some(windows_source_device)) => app::apply::ManualApplySelection::from_raw(
            adapter.as_deref(),
            target_device,
            windows_source_device,
        )
        .map(Some)
        .map_err(|error| error.to_string()),
        _ => Err("bluebond apply manual selection requires both --target-device and --windows-source-device"
            .to_string()),
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

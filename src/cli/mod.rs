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
    }
}

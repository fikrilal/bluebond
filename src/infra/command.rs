use std::env;
use std::path::Path;
use std::process::Command;

use crate::error::{BluebondError, Result};

#[derive(Debug)]
pub struct CommandOutput {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(program: &str, args: &[&str]) -> Result<CommandOutput> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|source| BluebondError::Io {
            context: "running command",
            source,
        })?;

    let command_output = CommandOutput {
        status: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    };

    if output.status.success() {
        Ok(command_output)
    } else {
        Err(BluebondError::CommandFailed {
            program: program.to_string(),
            status: command_output.status,
            stderr: command_output.stderr,
        })
    }
}

pub fn exists(program: &str) -> bool {
    if program.contains('/') {
        return Path::new(program).is_file();
    }

    env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| env::split_paths(&paths).collect::<Vec<_>>())
        .any(|path| path.join(program).is_file())
}

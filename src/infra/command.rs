use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

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
        command_failed(program, command_output)
    }
}

pub fn run_with_stdin(program: &str, args: &[&str], stdin: &str) -> Result<CommandOutput> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| BluebondError::Io {
            context: "running command",
            source,
        })?;

    {
        use std::io::Write;

        let Some(child_stdin) = child.stdin.as_mut() else {
            return Err(BluebondError::Io {
                context: "opening command stdin",
                source: std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "command stdin unavailable",
                ),
            });
        };

        child_stdin
            .write_all(stdin.as_bytes())
            .map_err(|source| BluebondError::Io {
                context: "writing command stdin",
                source,
            })?;
    }

    let output = child
        .wait_with_output()
        .map_err(|source| BluebondError::Io {
            context: "waiting for command",
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
        command_failed(program, command_output)
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

fn command_failed(program: &str, output: CommandOutput) -> Result<CommandOutput> {
    Err(BluebondError::CommandFailed {
        program: program.to_string(),
        status: output.status,
        stderr: output.stderr,
    })
}

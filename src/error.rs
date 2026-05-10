use thiserror::Error;

pub type Result<T> = std::result::Result<T, BluebondError>;

#[derive(Debug, Error)]
pub enum BluebondError {
    #[error("invalid Bluetooth address '{input}'")]
    InvalidBluetoothAddress { input: String },

    #[error("command failed: {program}")]
    CommandFailed {
        program: String,
        status: Option<i32>,
        stderr: String,
    },

    #[error("I/O error while {context}")]
    Io {
        context: &'static str,
        #[source]
        source: std::io::Error,
    },
}

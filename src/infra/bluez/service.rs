use crate::error::Result;
use crate::infra::command;

pub fn bluetoothctl_available() -> bool {
    command::exists("bluetoothctl")
}

pub fn systemctl_available() -> bool {
    command::exists("systemctl")
}

pub fn stop_bluetooth_service() -> Result<()> {
    command::run("systemctl", &["stop", "bluetooth.service"]).map(|_| ())
}

pub fn start_bluetooth_service() -> Result<()> {
    command::run("systemctl", &["start", "bluetooth.service"]).map(|_| ())
}

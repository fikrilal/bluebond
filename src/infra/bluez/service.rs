use crate::infra::command;

pub fn bluetoothctl_available() -> bool {
    command::exists("bluetoothctl")
}

pub fn systemctl_available() -> bool {
    command::exists("systemctl")
}

use crate::infra;

#[derive(Debug)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
}

impl DoctorReport {
    pub fn has_failures(&self) -> bool {
        self.checks.iter().any(|check| !check.ok)
    }
}

#[derive(Debug)]
pub struct DoctorCheck {
    pub name: &'static str,
    pub ok: bool,
    pub detail: &'static str,
}

pub fn run() -> DoctorReport {
    DoctorReport {
        checks: vec![
            DoctorCheck {
                name: "hivexget",
                ok: infra::windows::system_hive::hivexget_available(),
                detail: "required to read Windows SYSTEM registry hives",
            },
            DoctorCheck {
                name: "hivexsh",
                ok: infra::windows::system_hive::hivexsh_available(),
                detail: "required to traverse Windows SYSTEM registry hives",
            },
            DoctorCheck {
                name: "bluetoothctl",
                ok: infra::bluez::service::bluetoothctl_available(),
                detail: "required to verify BlueZ device state",
            },
            DoctorCheck {
                name: "systemctl",
                ok: infra::bluez::service::systemctl_available(),
                detail: "required for apply and rollback flows",
            },
            DoctorCheck {
                name: "BlueZ store",
                ok: infra::bluez::store::default_store_exists(),
                detail: "required to read Linux BlueZ bond records",
            },
            DoctorCheck {
                name: "findmnt",
                ok: infra::linux::mounts::findmnt_available(),
                detail: "used to discover mounted Windows partitions",
            },
        ],
    }
}

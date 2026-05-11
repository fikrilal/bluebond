use crate::infra::{
    self,
    bluez::store::{StoreReadiness, DEFAULT_BLUEZ_DIR},
};

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
    pub status: &'static str,
    pub detail: String,
}

pub fn run() -> DoctorReport {
    let bluez_store_readiness = infra::bluez::store::default_store_readiness();

    DoctorReport {
        checks: vec![
            command_check(
                "hivexget",
                infra::windows::system_hive::hivexget_available(),
                "required to read Windows SYSTEM registry hives",
            ),
            command_check(
                "hivexsh",
                infra::windows::system_hive::hivexsh_available(),
                "required to traverse Windows SYSTEM registry hives",
            ),
            command_check(
                "bluetoothctl",
                infra::bluez::service::bluetoothctl_available(),
                "required to verify BlueZ device state",
            ),
            command_check(
                "systemctl",
                infra::bluez::service::systemctl_available(),
                "required for apply and rollback flows",
            ),
            bluez_store_check(bluez_store_readiness),
            command_check(
                "findmnt",
                infra::linux::mounts::findmnt_available(),
                "used to discover mounted Windows partitions",
            ),
        ],
    }
}

fn command_check(name: &'static str, ok: bool, detail: &'static str) -> DoctorCheck {
    DoctorCheck {
        name,
        ok,
        status: if ok { "ok" } else { "missing" },
        detail: detail.to_string(),
    }
}

fn bluez_store_check(readiness: StoreReadiness) -> DoctorCheck {
    let (ok, status, detail) = match readiness {
        StoreReadiness::Readable => (
            true,
            "ok",
            format!("readable Linux BlueZ bond records at {DEFAULT_BLUEZ_DIR}"),
        ),
        StoreReadiness::Missing => (
            false,
            "missing",
            format!("missing {DEFAULT_BLUEZ_DIR}; BlueZ may not be installed or initialized"),
        ),
        StoreReadiness::NotDirectory => (
            false,
            "fail",
            format!("{DEFAULT_BLUEZ_DIR} exists but is not a directory"),
        ),
        StoreReadiness::PermissionDenied => (
            false,
            "no read",
            format!("{DEFAULT_BLUEZ_DIR} is not readable by this user; run scan/plan/dry-run with sudo or pass --bluez-dir to a readable fixture/export"),
        ),
        StoreReadiness::Unreadable => (
            false,
            "fail",
            format!("{DEFAULT_BLUEZ_DIR} exists but could not be read"),
        ),
    };

    DoctorCheck {
        name: "BlueZ store",
        ok,
        status,
        detail,
    }
}

use crate::domain::{
    BluetoothAddress, DiscoveredBondState, KeyMaterialPresence, LinuxBondDevice,
    WindowsBondAdapter, WindowsBondDevice,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BondMatchReport {
    pub adapters: Vec<AdapterMatch>,
}

impl BondMatchReport {
    pub fn exact_from(state: &DiscoveredBondState) -> Self {
        let adapters = state
            .linux_adapters
            .iter()
            .map(|linux_adapter| {
                let windows_adapter = state
                    .windows_adapters
                    .iter()
                    .find(|adapter| adapter.address == linux_adapter.address);

                match windows_adapter {
                    Some(windows_adapter) => {
                        let devices = linux_adapter
                            .devices
                            .iter()
                            .map(|linux_device| {
                                let windows_device = windows_adapter
                                    .devices
                                    .iter()
                                    .find(|device| device.address == linux_device.address);

                                DeviceMatch::from_windows_adapter(
                                    linux_device,
                                    windows_adapter,
                                    windows_device,
                                )
                            })
                            .collect();

                        AdapterMatch {
                            linux_address: linux_adapter.address,
                            windows_address: Some(windows_adapter.address),
                            status: AdapterMatchStatus::Exact,
                            devices,
                        }
                    }
                    None => AdapterMatch {
                        linux_address: linux_adapter.address,
                        windows_address: None,
                        status: AdapterMatchStatus::MissingWindowsAdapter,
                        devices: linux_adapter
                            .devices
                            .iter()
                            .map(DeviceMatch::missing_windows_adapter)
                            .collect(),
                    },
                }
            })
            .collect();

        Self { adapters }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdapterMatch {
    pub linux_address: BluetoothAddress,
    pub windows_address: Option<BluetoothAddress>,
    pub status: AdapterMatchStatus,
    pub devices: Vec<DeviceMatch>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AdapterMatchStatus {
    Exact,
    MissingWindowsAdapter,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeviceMatch {
    pub linux_address: BluetoothAddress,
    pub windows_address: Option<BluetoothAddress>,
    pub display_name: String,
    pub status: DeviceMatchStatus,
    pub drift_candidates: Vec<DeviceDriftCandidate>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeviceDriftCandidate {
    pub windows_address: BluetoothAddress,
    pub differing_bytes: u8,
    pub key_material: KeyMaterialPresence,
}

impl DeviceMatch {
    fn from_windows_adapter(
        linux_device: &LinuxBondDevice,
        windows_adapter: &WindowsBondAdapter,
        windows_device: Option<&WindowsBondDevice>,
    ) -> Self {
        match windows_device {
            Some(device) => Self::from_exact_match(linux_device, device),
            None => Self::from_drift_candidates(linux_device, windows_adapter),
        }
    }

    fn from_exact_match(
        linux_device: &LinuxBondDevice,
        windows_device: &WindowsBondDevice,
    ) -> Self {
        let status = if windows_device.key_material == KeyMaterialPresence::Present {
            DeviceMatchStatus::ExactUsable
        } else {
            DeviceMatchStatus::ExactMissingWindowsKeyMaterial
        };

        Self {
            linux_address: linux_device.address,
            windows_address: Some(windows_device.address),
            display_name: linux_device.display_name.clone(),
            status,
            drift_candidates: Vec::new(),
        }
    }

    fn from_drift_candidates(
        linux_device: &LinuxBondDevice,
        windows_adapter: &WindowsBondAdapter,
    ) -> Self {
        let drift_candidates = windows_adapter
            .devices
            .iter()
            .filter_map(|device| {
                let differing_bytes = differing_byte_count(linux_device.address, device.address);

                if differing_bytes == 1 && device.key_material == KeyMaterialPresence::Present {
                    Some(DeviceDriftCandidate {
                        windows_address: device.address,
                        differing_bytes,
                        key_material: device.key_material,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let status = match drift_candidates.len() {
            0 => DeviceMatchStatus::MissingWindowsDevice,
            1 => DeviceMatchStatus::AddressDriftCandidate,
            _ => DeviceMatchStatus::AmbiguousAddressDrift,
        };

        Self {
            linux_address: linux_device.address,
            windows_address: if drift_candidates.len() == 1 {
                Some(drift_candidates[0].windows_address)
            } else {
                None
            },
            display_name: linux_device.display_name.clone(),
            status,
            drift_candidates,
        }
    }

    fn missing_windows_adapter(linux_device: &LinuxBondDevice) -> Self {
        Self {
            linux_address: linux_device.address,
            windows_address: None,
            display_name: linux_device.display_name.clone(),
            status: DeviceMatchStatus::MissingWindowsAdapter,
            drift_candidates: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceMatchStatus {
    ExactUsable,
    ExactMissingWindowsKeyMaterial,
    AddressDriftCandidate,
    AmbiguousAddressDrift,
    MissingWindowsDevice,
    MissingWindowsAdapter,
}

fn differing_byte_count(left: BluetoothAddress, right: BluetoothAddress) -> u8 {
    left.bytes()
        .into_iter()
        .zip(right.bytes())
        .filter(|(left, right)| left != right)
        .count() as u8
}

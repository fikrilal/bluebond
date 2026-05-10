use crate::domain::{
    BluetoothAddress, DiscoveredBondState, KeyMaterialPresence, LinuxBondDevice, WindowsBondDevice,
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

                                DeviceMatch::from_exact_match(linux_device, windows_device)
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
}

impl DeviceMatch {
    fn from_exact_match(
        linux_device: &LinuxBondDevice,
        windows_device: Option<&WindowsBondDevice>,
    ) -> Self {
        let status = match windows_device {
            Some(device) if device.key_material == KeyMaterialPresence::Present => {
                DeviceMatchStatus::ExactUsable
            }
            Some(_) => DeviceMatchStatus::ExactMissingWindowsKeyMaterial,
            None => DeviceMatchStatus::MissingWindowsDevice,
        };

        Self {
            linux_address: linux_device.address,
            windows_address: windows_device.map(|device| device.address),
            display_name: linux_device.display_name.clone(),
            status,
        }
    }

    fn missing_windows_adapter(linux_device: &LinuxBondDevice) -> Self {
        Self {
            linux_address: linux_device.address,
            windows_address: None,
            display_name: linux_device.display_name.clone(),
            status: DeviceMatchStatus::MissingWindowsAdapter,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceMatchStatus {
    ExactUsable,
    ExactMissingWindowsKeyMaterial,
    MissingWindowsDevice,
    MissingWindowsAdapter,
}

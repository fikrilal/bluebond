use crate::domain::BluetoothAddress;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiscoveredBondState {
    pub linux_adapters: Vec<LinuxBondAdapter>,
    pub windows_adapters: Vec<WindowsBondAdapter>,
}

impl DiscoveredBondState {
    pub fn new(
        mut linux_adapters: Vec<LinuxBondAdapter>,
        mut windows_adapters: Vec<WindowsBondAdapter>,
    ) -> Self {
        linux_adapters.sort_by_key(|adapter| adapter.address);
        windows_adapters.sort_by_key(|adapter| adapter.address);

        Self {
            linux_adapters,
            windows_adapters,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxBondAdapter {
    pub address: BluetoothAddress,
    pub devices: Vec<LinuxBondDevice>,
}

impl LinuxBondAdapter {
    pub fn new(address: BluetoothAddress, mut devices: Vec<LinuxBondDevice>) -> Self {
        devices.sort_by_key(|device| device.address);

        Self { address, devices }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxBondDevice {
    pub address: BluetoothAddress,
    pub display_name: String,
    pub address_type: Option<String>,
    pub paired: Option<bool>,
    pub trusted: Option<bool>,
    pub key_material: LinuxKeyMaterial,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxKeyMaterial {
    pub link_key: KeyMaterialPresence,
    pub long_term_key: KeyMaterialPresence,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsBondAdapter {
    pub address: BluetoothAddress,
    pub control_set: String,
    pub registry_path: String,
    pub devices: Vec<WindowsBondDevice>,
}

impl WindowsBondAdapter {
    pub fn new(
        address: BluetoothAddress,
        control_set: impl Into<String>,
        registry_path: impl Into<String>,
        mut devices: Vec<WindowsBondDevice>,
    ) -> Self {
        devices.sort_by_key(|device| device.address);

        Self {
            address,
            control_set: control_set.into(),
            registry_path: registry_path.into(),
            devices,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsBondDevice {
    pub address: BluetoothAddress,
    pub registry_path: String,
    pub key_material: KeyMaterialPresence,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeyMaterialPresence {
    Present,
    NotPresent,
}

impl KeyMaterialPresence {
    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Present
        } else {
            Self::NotPresent
        }
    }
}

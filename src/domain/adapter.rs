use crate::domain::BluetoothAddress;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothAdapter {
    pub address: BluetoothAddress,
    pub devices: Vec<BluetoothDevice>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothDevice {
    pub address: BluetoothAddress,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub address_type: Option<String>,
    pub paired: Option<bool>,
    pub trusted: Option<bool>,
    pub has_link_key: bool,
    pub has_long_term_key: bool,
}

impl BluetoothDevice {
    pub fn display_name(&self) -> &str {
        self.alias
            .as_deref()
            .or(self.name.as_deref())
            .unwrap_or("(unnamed)")
    }
}

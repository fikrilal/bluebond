use std::path::Path;

pub const DEFAULT_BLUEZ_DIR: &str = "/var/lib/bluetooth";

pub fn default_store_exists() -> bool {
    Path::new(DEFAULT_BLUEZ_DIR).exists()
}

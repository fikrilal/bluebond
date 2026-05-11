mod adapter;
mod address;
mod bond_state;
mod matching;
mod plan;

pub use adapter::{BluetoothAdapter, BluetoothDevice};
pub use address::BluetoothAddress;
pub use bond_state::{
    DiscoveredBondState, KeyMaterialPresence, LinuxBondAdapter, LinuxBondDevice, LinuxKeyMaterial,
    WindowsBondAdapter, WindowsBondDevice,
};
pub use matching::{
    AdapterMatch, AdapterMatchStatus, BondMatchReport, DeviceMatch, DeviceMatchStatus,
};
pub use plan::{SkipReason, SkippedSyncCandidate, SyncPlan, SyncPlanAction, SyncPlanActionType};

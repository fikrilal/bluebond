use crate::domain::{BluetoothAddress, BondMatchReport, DeviceMatch, DeviceMatchStatus};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SyncPlan {
    pub actions: Vec<SyncPlanAction>,
    pub skipped: Vec<SkippedSyncCandidate>,
}

impl SyncPlan {
    pub fn from_match_report(report: &BondMatchReport) -> Self {
        let mut actions = Vec::new();
        let mut skipped = Vec::new();

        for adapter in &report.adapters {
            for device in &adapter.devices {
                match SyncPlanAction::from_device_match(adapter.linux_address, device) {
                    Some(action) => actions.push(action),
                    None => skipped.push(SkippedSyncCandidate::from_device_match(
                        adapter.linux_address,
                        device,
                    )),
                }
            }
        }

        Self { actions, skipped }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SyncPlanAction {
    pub action_type: SyncPlanActionType,
    pub linux_adapter_address: BluetoothAddress,
    pub linux_target_device_address: BluetoothAddress,
    pub bluez_template_device_address: Option<BluetoothAddress>,
    pub windows_source_device_address: BluetoothAddress,
    pub display_name: String,
}

impl SyncPlanAction {
    fn from_device_match(
        linux_adapter_address: BluetoothAddress,
        device: &DeviceMatch,
    ) -> Option<Self> {
        let action_type = match device.status {
            DeviceMatchStatus::ExactUsable => SyncPlanActionType::UpdateExistingBluezRecord,
            DeviceMatchStatus::AddressDriftCandidate => SyncPlanActionType::CreateBluezRecord,
            DeviceMatchStatus::ExactMissingWindowsKeyMaterial
            | DeviceMatchStatus::AmbiguousAddressDrift
            | DeviceMatchStatus::MissingWindowsDevice
            | DeviceMatchStatus::MissingWindowsAdapter => return None,
        };

        Some(Self {
            action_type,
            linux_adapter_address,
            linux_target_device_address: device.linux_address,
            bluez_template_device_address: None,
            windows_source_device_address: device.windows_address?,
            display_name: device.display_name.clone(),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SyncPlanActionType {
    UpdateExistingBluezRecord,
    CreateBluezRecord,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SkippedSyncCandidate {
    pub linux_adapter_address: BluetoothAddress,
    pub linux_device_address: BluetoothAddress,
    pub display_name: String,
    pub reason: SkipReason,
}

impl SkippedSyncCandidate {
    fn from_device_match(linux_adapter_address: BluetoothAddress, device: &DeviceMatch) -> Self {
        Self {
            linux_adapter_address,
            linux_device_address: device.linux_address,
            display_name: device.display_name.clone(),
            reason: SkipReason::from_device_status(device.status),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SkipReason {
    MissingWindowsAdapter,
    MissingWindowsDevice,
    MissingWindowsKeyMaterial,
    AmbiguousAddressDrift,
}

impl SkipReason {
    fn from_device_status(status: DeviceMatchStatus) -> Self {
        match status {
            DeviceMatchStatus::ExactMissingWindowsKeyMaterial => Self::MissingWindowsKeyMaterial,
            DeviceMatchStatus::AmbiguousAddressDrift => Self::AmbiguousAddressDrift,
            DeviceMatchStatus::MissingWindowsDevice => Self::MissingWindowsDevice,
            DeviceMatchStatus::MissingWindowsAdapter => Self::MissingWindowsAdapter,
            DeviceMatchStatus::ExactUsable | DeviceMatchStatus::AddressDriftCandidate => {
                unreachable!("usable matches are not skipped")
            }
        }
    }
}

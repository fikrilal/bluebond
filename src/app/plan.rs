use std::path::{Path, PathBuf};

use crate::app::scan::ScanReport;
use crate::domain::{BondMatchReport, SkipReason, SyncPlan, SyncPlanAction, SyncPlanActionType};
use crate::infra::bluez::store;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenderPlanRequest {
    pub bluez_dir: PathBuf,
}

impl RenderPlanRequest {
    pub fn new(bluez_dir: impl Into<PathBuf>) -> Self {
        Self {
            bluez_dir: bluez_dir.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenderedSyncPlan {
    pub bluez_dir: PathBuf,
    pub changes: Vec<RenderedSyncChange>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlanReport {
    pub rendered_plan: RenderedSyncPlan,
    pub skipped: Vec<RenderedSkippedCandidate>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenderedSkippedCandidate {
    pub linux_adapter_address: String,
    pub linux_device_address: String,
    pub display_name: String,
    pub reason: RenderedSkipReason,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderedSkipReason {
    MissingWindowsAdapter,
    MissingWindowsDevice,
    MissingWindowsKeyMaterial,
    AmbiguousAddressDrift,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenderedSyncChange {
    pub change_type: RenderedSyncChangeType,
    pub display_name: String,
    pub linux_adapter_address: String,
    pub linux_target_device_address: String,
    pub windows_source_device_address: String,
    pub target_device_dir: PathBuf,
    pub target_info_path: PathBuf,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderedSyncChangeType {
    CreateBluezRecord,
    UpdateBluezRecord,
}

pub fn render(plan: &SyncPlan, request: &RenderPlanRequest) -> RenderedSyncPlan {
    let changes = plan
        .actions
        .iter()
        .map(|action| render_action(action, &request.bluez_dir))
        .collect();

    RenderedSyncPlan {
        bluez_dir: request.bluez_dir.clone(),
        changes,
    }
}

pub fn build_plan(scan_report: &ScanReport, request: &RenderPlanRequest) -> PlanReport {
    let bond_state = scan_report.discovered_bond_state();
    let match_report = BondMatchReport::exact_from(&bond_state);
    let sync_plan = SyncPlan::from_match_report(&match_report);
    let rendered_plan = render(&sync_plan, request);
    let skipped = sync_plan
        .skipped
        .iter()
        .map(|skipped| RenderedSkippedCandidate {
            linux_adapter_address: skipped.linux_adapter_address.to_string(),
            linux_device_address: skipped.linux_device_address.to_string(),
            display_name: skipped.display_name.clone(),
            reason: render_skip_reason(skipped.reason),
        })
        .collect();

    PlanReport {
        rendered_plan,
        skipped,
    }
}

pub fn default_request() -> RenderPlanRequest {
    RenderPlanRequest::new(Path::new(store::DEFAULT_BLUEZ_DIR))
}

fn render_action(action: &SyncPlanAction, bluez_dir: &Path) -> RenderedSyncChange {
    let target_device_dir = bluez_dir
        .join(action.linux_adapter_address.to_string())
        .join(action.linux_target_device_address.to_string());
    let target_info_path = target_device_dir.join("info");

    RenderedSyncChange {
        change_type: render_change_type(action.action_type),
        display_name: action.display_name.clone(),
        linux_adapter_address: action.linux_adapter_address.to_string(),
        linux_target_device_address: action.linux_target_device_address.to_string(),
        windows_source_device_address: action.windows_source_device_address.to_string(),
        target_device_dir,
        target_info_path,
    }
}

fn render_change_type(action_type: SyncPlanActionType) -> RenderedSyncChangeType {
    match action_type {
        SyncPlanActionType::CreateBluezRecord => RenderedSyncChangeType::CreateBluezRecord,
        SyncPlanActionType::UpdateExistingBluezRecord => RenderedSyncChangeType::UpdateBluezRecord,
    }
}

fn render_skip_reason(reason: SkipReason) -> RenderedSkipReason {
    match reason {
        SkipReason::MissingWindowsAdapter => RenderedSkipReason::MissingWindowsAdapter,
        SkipReason::MissingWindowsDevice => RenderedSkipReason::MissingWindowsDevice,
        SkipReason::MissingWindowsKeyMaterial => RenderedSkipReason::MissingWindowsKeyMaterial,
        SkipReason::AmbiguousAddressDrift => RenderedSkipReason::AmbiguousAddressDrift,
    }
}

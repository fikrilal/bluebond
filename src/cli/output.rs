use crate::app::doctor::DoctorReport;
use crate::app::plan::{PlanReport, RenderedSkipReason, RenderedSyncChangeType};
use crate::app::scan::{ScanReport, WindowsBluetoothKeyInspectionStatus, WindowsSystemHiveStatus};

pub fn print_doctor_report(report: &DoctorReport) {
    println!("BlueBond doctor\n");

    for check in &report.checks {
        let status = if check.ok { "ok" } else { "missing" };
        println!("{status:>7}  {:<20} {}", check.name, check.detail);
    }
}

pub fn print_scan_report(report: &ScanReport) {
    println!("BlueBond scan\n");
    println!("BlueZ store: {}", report.bluez_dir.display());
    println!();
    print_windows_candidates(report);
    println!();
    print_windows_bluetooth_keys(report);
    println!();

    if report.adapters.is_empty() {
        println!("No BlueZ adapters found.");
        return;
    }

    println!("Linux adapters:");

    for (adapter_index, adapter) in report.adapters.iter().enumerate() {
        println!(
            "  [{}] {} ({} device{})",
            adapter_index + 1,
            adapter.address,
            adapter.devices.len(),
            plural(adapter.devices.len())
        );

        for (device_index, device) in adapter.devices.iter().enumerate() {
            println!(
                "      [{}] {}  {}",
                device_index + 1,
                device.address,
                device.display_name()
            );
            println!(
                "          paired: {}  trusted: {}  address type: {}  keys: {}",
                format_optional_bool(device.paired),
                format_optional_bool(device.trusted),
                device.address_type.as_deref().unwrap_or("unknown"),
                format_key_summary(device.has_link_key, device.has_long_term_key)
            );
        }
    }
}

pub fn print_plan_report(report: &PlanReport) {
    println!("BlueBond plan\n");
    println!("BlueZ store: {}", report.rendered_plan.bluez_dir.display());
    println!();

    println!("Planned changes:");
    if report.rendered_plan.changes.is_empty() {
        println!("  none");
    } else {
        for (index, change) in report.rendered_plan.changes.iter().enumerate() {
            println!(
                "  [{}] {}  {}",
                index + 1,
                format_rendered_change_type(change.change_type),
                change.display_name
            );
            println!("      Linux adapter: {}", change.linux_adapter_address);
            println!(
                "      Linux target:  {}",
                change.linux_target_device_address
            );
            println!(
                "      Windows source: {}",
                change.windows_source_device_address
            );
            println!(
                "      Target dir:    {}",
                change.target_device_dir.display()
            );
            println!("      Target info:   {}", change.target_info_path.display());
        }
    }

    println!();
    println!("Skipped candidates:");
    if report.skipped.is_empty() {
        println!("  none");
    } else {
        for (index, skipped) in report.skipped.iter().enumerate() {
            println!(
                "  [{}] {}  {}",
                index + 1,
                skipped.linux_device_address,
                skipped.display_name
            );
            println!("      Linux adapter: {}", skipped.linux_adapter_address);
            println!("      Reason: {}", format_skip_reason(skipped.reason));
        }
    }

    println!();
    println!("No changes made.");
}

fn print_windows_bluetooth_keys(report: &ScanReport) {
    println!("Windows Bluetooth keys:");

    if report.windows_bluetooth_keys.is_empty() {
        println!("  not inspected");
        return;
    }

    for inspection in &report.windows_bluetooth_keys {
        println!(
            "  hive: {} ({})",
            inspection.hive_path.display(),
            format_windows_bluetooth_key_status(&inspection.status)
        );

        if inspection.adapters.is_empty() {
            continue;
        }

        for (index, adapter) in inspection.adapters.iter().enumerate() {
            println!("      [{}] {}", index + 1, adapter.adapter_address);
            println!("          source: {}", adapter.registry_path);

            if adapter.devices.is_empty() {
                println!("          devices: not found");
                continue;
            }

            println!("          devices:");
            for (device_index, device) in adapter.devices.iter().enumerate() {
                println!(
                    "            [{}] {}  key material: {}",
                    device_index + 1,
                    device.device_address,
                    format_bool_present(device.has_key_material)
                );
                println!("                source: {}", device.registry_path);
            }
        }
    }
}

fn print_windows_candidates(report: &ScanReport) {
    println!("Windows installations:");

    if report.windows_candidates.is_empty() {
        println!("  none detected");
        return;
    }

    for (index, candidate) in report.windows_candidates.iter().enumerate() {
        println!(
            "  [{}] {}",
            index + 1,
            format_windows_status(&candidate.status)
        );
        println!("      root: {}", candidate.root.display());
        println!("      hive: {}", candidate.hive_path.display());
    }
}

fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn format_optional_bool(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unknown",
    }
}

fn format_key_summary(has_link_key: bool, has_long_term_key: bool) -> &'static str {
    match (has_link_key, has_long_term_key) {
        (true, true) => "link+ltk",
        (true, false) => "link",
        (false, true) => "ltk",
        (false, false) => "not present",
    }
}

fn format_bool_present(value: bool) -> &'static str {
    if value {
        "present"
    } else {
        "not present"
    }
}

fn format_windows_status(status: &WindowsSystemHiveStatus) -> &'static str {
    match status {
        WindowsSystemHiveStatus::Ready => "ready",
        WindowsSystemHiveStatus::Missing => "missing SYSTEM hive",
        WindowsSystemHiveStatus::NotFile => "SYSTEM hive path is not a file",
        WindowsSystemHiveStatus::Unreadable => "SYSTEM hive is not readable",
    }
}

fn format_windows_bluetooth_key_status(
    status: &WindowsBluetoothKeyInspectionStatus,
) -> &'static str {
    match status {
        WindowsBluetoothKeyInspectionStatus::Ready => "ready",
        WindowsBluetoothKeyInspectionStatus::MissingTool => "hivexsh missing",
        WindowsBluetoothKeyInspectionStatus::NoKeysFound => "no adapter keys found",
        WindowsBluetoothKeyInspectionStatus::CommandFailed => "registry inspection failed",
    }
}

fn format_rendered_change_type(change_type: RenderedSyncChangeType) -> &'static str {
    match change_type {
        RenderedSyncChangeType::CreateBluezRecord => "create BlueZ record",
        RenderedSyncChangeType::UpdateBluezRecord => "update BlueZ record",
    }
}

fn format_skip_reason(reason: RenderedSkipReason) -> &'static str {
    match reason {
        RenderedSkipReason::MissingWindowsAdapter => "missing Windows adapter",
        RenderedSkipReason::MissingWindowsDevice => "missing Windows device",
        RenderedSkipReason::MissingWindowsKeyMaterial => "missing Windows key material",
        RenderedSkipReason::AmbiguousAddressDrift => "ambiguous address drift",
    }
}

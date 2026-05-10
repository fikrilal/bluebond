use crate::app::doctor::DoctorReport;
use crate::app::scan::ScanReport;

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

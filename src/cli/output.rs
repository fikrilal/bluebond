use crate::app::doctor::DoctorReport;

pub fn print_doctor_report(report: &DoctorReport) {
    println!("BlueBond doctor\n");

    for check in &report.checks {
        let status = if check.ok { "ok" } else { "missing" };
        println!("{status:>7}  {:<20} {}", check.name, check.detail);
    }
}

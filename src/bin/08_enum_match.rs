enum PatientStatus { Waiting, Admitted, Discharged }
fn status_label(status: PatientStatus) -> &'static str {
    match status {
        PatientStatus::Waiting => "Waiting",
        PatientStatus::Admitted => "Admitted",
        PatientStatus::Discharged => "Discharged",
    }
}
fn main() {
    println!("=== 08 Enum & Match ===");
    for status in [PatientStatus::Waiting, PatientStatus::Admitted, PatientStatus::Discharged] {
        println!("{}", status_label(status));
    }
    // TRY: add a Cancelled variant and update match.
}

#[derive(Debug)]
struct Patient { id: u32, name: String, age: u8 }
impl Patient {
    fn describe(&self) -> String { format!("#{} {} ({} years)", self.id, self.name, self.age) }
}
fn main() {
    println!("=== 07 Struct & Impl ===");
    let patient = Patient { id: 1, name: String::from("Somchai"), age: 45 };
    println!("{}", patient.describe());
    println!("{patient:?}");
    // TRY: add a diagnosis field to Patient.
}

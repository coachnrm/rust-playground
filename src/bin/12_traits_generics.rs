trait Describe { fn describe(&self) -> String; }
struct Doctor { name: String }
impl Describe for Doctor {
    fn describe(&self) -> String { format!("Doctor: {}", self.name) }
}
fn print_description<T: Describe>(item: &T) { println!("{}", item.describe()); }
fn main() {
    println!("=== 12 Traits & Generics ===");
    let doctor = Doctor { name: String::from("Somchai") };
    print_description(&doctor);
    // TRY: create a Nurse struct implementing Describe.
}

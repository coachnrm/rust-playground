
trait Describe {
    fn describe(&self) -> String;
}

// Doctor
struct Doctor {
    name: String,
    specialty: String,
}

impl Describe for Doctor {
    fn describe(&self) -> String {
        format!(
            "Doctor: {}, Specialty: {}",
            self.name, self.specialty
        )
    }
}

// Nurse
struct Nurse {
    name: String,
    ward: String,
}

impl Describe for Nurse {
    fn describe(&self) -> String {
        format!(
            "Nurse: {}, Ward: {}",
            self.name, self.ward
        )
    }
}

// Pharmacist
struct Pharmacist {
    name: String,
}

impl Describe for Pharmacist {
    fn describe(&self) -> String {
        format!("Pharmacist: {}", self.name)
    }
}

fn main() {
    println!("=== 13 Dynamic Dispatch ===");

    // Vec เก็บ Struct ต่างชนิดผ่าน Trait Object
    let staff: Vec<Box<dyn Describe>> = vec![
        Box::new(Doctor {
            name: String::from("Somchai"),
            specialty: String::from("Cardiothoracic Surgery"),
        }),
        Box::new(Nurse {
            name: String::from("Suda"),
            ward: String::from("ICU"),
        }),
        Box::new(Pharmacist {
            name: String::from("Nok"),
        }),
    ];

    // เรียก Method ผ่าน dyn Trait
    for person in &staff {
        println!("{}", person.describe());
    }

    println!("Total staff: {}", staff.len());
}

//static dispatch vs dynamic dispatch
fn main() {
    println!("=== 02 Data Types ===");
    let age: i32 = 35;
    let weight: f64 = 72.5;
    let active: bool = true;
    let grade: char = 'A';
    let name: &str = "Somchai";
    let numbers: [i32; 3] = [10, 20, 30];
    let person: (&str, u8) = ("Nok", 25);
    println!("{name}: age={age}, weight={weight}, active={active}, grade={grade}");
    println!("array={numbers:?}, tuple={person:?}");
    // TRY: add a String::from("Hello") variable.
}

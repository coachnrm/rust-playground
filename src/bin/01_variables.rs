fn main() {
    println!("=== 01 Variables & Mutability ===");
    let x = 10;
    let mut y = 20;
    y += 5;
    println!("x = {x}, y = {y}");
    let x = "Rust"; // shadowing
    println!("shadowed x = {x}");
    // TRY: change y to 100, then run again.
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn greet(name: &str) { println!("Hello, {name}!"); }
fn main() {
    println!("=== 05 Functions ===");
    greet("Rust");
    println!("10 + 20 = {}", add(10, 20));
    // TRY: write fn multiply(a: i32, b: i32) -> i32.
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err(String::from("division by zero")) } else { Ok(a / b) }
}
fn main() {
    println!("=== 10 Option & Result ===");
    let values = [10, 20, 30];
    match values.get(5) { Some(v) => println!("value={v}"), None => println!("index not found") }
    for divisor in [2.0, 0.0] {
        match divide(10.0, divisor) { Ok(v) => println!("result={v}"), Err(e) => println!("error={e}") }
    }
    // TRY: change the index from 5 to 1.
}

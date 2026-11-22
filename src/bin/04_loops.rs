fn main() {
    println!("=== 04 Loops ===");
    for i in 1..=5 { println!("for: {i}"); }
    let mut n = 0;
    while n < 3 { println!("while: {n}"); n += 1; }
    let mut count = 0;
    let result = loop { count += 1; if count == 4 { break count * 2; } };
    println!("loop returned {result}");
    // TRY: print even numbers from 2 to 20.
}

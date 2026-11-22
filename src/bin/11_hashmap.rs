use std::collections::HashMap;
fn main() {
    println!("=== 11 HashMap ===");
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    println!("Alice = {:?}", scores.get("Alice"));
    for (name, score) in &scores { println!("{name}: {score}"); }
    // TRY: update Bob's score.
}

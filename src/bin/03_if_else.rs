fn main() {
    println!("=== 03 If / Else ===");
    let score = 85;
    let grade = if score >= 80 { "A" } else if score >= 70 { "B" } else { "C" };
    println!("score={score}, grade={grade}");
    // TRY: set score to 65 and compare results.
}

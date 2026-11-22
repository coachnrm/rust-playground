
fn main() {
    println!("=== 03 Match ===");

    let score = 85;

    let grade = match score {
        80..=100 => "A",
        70..=79 => "B",
        0..=69 => "C",
        _ => "Invalid score",
    };

    println!("score={score}, grade={grade}");
}

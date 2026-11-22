fn print_borrowed(value: &str) { println!("borrowed: {value}"); }
fn main() {
    println!("=== 06 Ownership & Borrowing ===");
    let s1 = String::from("Hello Rust");
    let s2 = s1; // ownership moves; s1 cannot be used now
    println!("owner s2: {s2}");
    let mut s3 = String::from("Borrowing");
    print_borrowed(&s3); // immutable borrow
    s3.push_str(" is useful");
    println!("still own s3: {s3}");
    // TRY: uncomment println!("{s1}"); and inspect compiler error.
}

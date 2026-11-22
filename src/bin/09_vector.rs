fn main() {
    println!("=== 09 Vector ===");
    let mut numbers = vec![10, 20, 30];
    numbers.push(40);
    for (index, number) in numbers.iter().enumerate() {
        println!("index={index}, number={number}");
    }
    println!("count={}, first={:?}", numbers.len(), numbers.first());
    // TRY: use numbers.pop() and print the result.
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];

    for name in names {
        println!("{}", greet(name));
    }

    // 簡単な計算
    let sum: i32 = (1..=5).sum();
    println!("Sum of 1 to 5: {}", sum);
}

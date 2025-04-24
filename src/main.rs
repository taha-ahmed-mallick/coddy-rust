use std::io;

fn analyze_string(s: &str) {
    // Write your code here
    println!("Length: {}", s.len());
    println!("Char at 4: {}", s.chars().nth(4).unwrap());
    println!("Contains Rust: {}", s.contains("Rust"));
    println!("Ends with dot: {}", s.ends_with("."));
    println!("Uppercase: {}", s.to_uppercase());
}

fn main() {
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();
    analyze_string(message);
}

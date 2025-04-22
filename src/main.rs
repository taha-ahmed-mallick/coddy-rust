use std::io;

fn sigma(n: i32) -> i32 {
    // Write your code below
    return n * (n + 1) / 2;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let res = sigma(n);
    println!("{}", res);
}

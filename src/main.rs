use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();
    let mut num: i32;
    // Write your code below
    let mut sum: i32 = 0;
    for _i in 0..count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        num = input.trim().parse().unwrap();
        sum += num;
    }
    println!("{}", sum);
}

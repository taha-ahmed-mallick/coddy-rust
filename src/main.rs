use std::io;

fn main() {
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).unwrap();
    let n: i32 = input_n.trim().parse().unwrap();

    for i in 1..i32::MAX {
        let line = "*".repeat((2*i-1) as usize);
        println!("{}", line);
        if n == (2*i-1) {
            break;
        }
    }
}
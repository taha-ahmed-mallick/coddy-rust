use std::convert::TryInto;
use std::io;

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let trimmed = input_str_arr.trim();
    let arr: [String; 5] = trimmed
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();

    print!("[");
    // Write your code below
    for i in 0..arr.len() {
        if i == arr.len() - 1 {
            print!("{}", arr[i]);
        } else {
            print!("{}, ", arr[i]);
        }
    }

    println!("]");
}

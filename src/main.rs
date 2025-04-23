use std::io;
use std::convert::TryInto;

fn rev_arr(arr: &[i32]) -> [i32; 8] {
    // Write your code below
    let mut rev = [0; 8];
    for (i, &val) in arr.iter().rev().enumerate() {
        rev[i] = val;
    }
   return rev;
}

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let input_str_arr = input_str_arr.trim();
    let numbers: [i32; 8] = input_str_arr.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let result = rev_arr(&numbers);
    println!("The reversed array is: {:?}", result);
}
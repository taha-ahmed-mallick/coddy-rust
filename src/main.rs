use std::io;
use std::convert::TryInto;

fn calculate_stats(arr: [i32; 8]) -> [f64; 4] {
    // Write your code here
    let mut sum = 0;
    for number in &arr {
        sum+=number;
    }
    let avg = sum as f64/arr.len() as f64;
    let mut max = arr[0];
    for &number in &arr[1..] {
        if number > max {
            max = number;
        }
    }
    let mut min = arr[0];
    for &number in &arr[1..] {
        if number < min {
            min = number;
        }
    }
    return [sum as f64, avg, max as f64, min as f64];
}

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let input_str_arr = input_str_arr.trim();
    let numbers: [i32; 8] = input_str_arr.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let stats = calculate_stats(numbers);
    println!("Sum: {}", stats[0]);
    println!("Average: {}", stats[1]);
    println!("Maximum: {}", stats[2]);
    println!("Minimum: {}", stats[3]);
}
use std::io;
use std::convert::TryInto;


fn calculate_average_grade(grades: [i32; 8]) -> String {
    // Write your code below
    let mut  sum: i32 = 0;
    for marks in grades.iter() {
        sum+=marks;
    }
    let avg: f64 = (sum as f64) / 8.0;
    let grade: char = if avg>=90.0 {
        'A'
    } else if avg>=80.0 {
        'B'
    } else if avg >=70.0 {
        'C'
    } else if avg >=60.0{
        'D'
    } else {
        'F'
    };
    let res = format!("Average grade: {:.2} - {grade}", avg);
    return res;
}
fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let input_str_arr = input_str_arr.trim();
    let arr: [i32; 8] = input_str_arr.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let res = calculate_average_grade(arr);
    println!("{}", res);
}
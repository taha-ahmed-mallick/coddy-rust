use std::io;
use std::convert::TryInto;

fn main() {
    let mut input_str_arr_1 = String::new();
    let mut input_str_arr_2 = String::new();
    io::stdin().read_line(&mut input_str_arr_1).unwrap();
    io::stdin().read_line(&mut input_str_arr_2).unwrap();
    let input_str_arr_1 = input_str_arr_1.trim();
    let input_str_arr_2 = input_str_arr_2.trim();
    let arr1: [String; 8] = input_str_arr_1.split(',').map(String::from).collect::<Vec<String>>().try_into().unwrap();
    let arr2: [String; 3] = input_str_arr_2.split(',').map(String::from).collect::<Vec<String>>().try_into().unwrap();
    
    let mut result = true;
    // Write your code below
    for (i, value)  in arr1.iter().enumerate() {
        if value == &arr2[0] {
            for (j,num) in arr2.iter().enumerate() {
                if num == &arr1[i+j] {
                    continue;
                }
                result = false;
            }
        }
    }


    println!("{}", result);
}
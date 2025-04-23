use std::io;
use std::convert::TryInto;

fn analyze_array(arr: &mut [String; 5]) {
    // Write code here
    println!("Is empty: {}", arr.is_empty());
    println!("Contains 5: {}", arr.contains(&"5".to_string()));
    println!("Reversed Elements:");
    arr.reverse();
    // Print elements line by line
    for element in arr.iter() {
        println!("{}", element);
    }
}

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let mut arr: [String; 5] = input_str_arr.split(',').map(String::from).collect::<Vec<String>>().try_into().unwrap();
    analyze_array(&mut arr);
}
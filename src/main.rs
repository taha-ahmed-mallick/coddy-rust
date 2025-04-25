use std::io;


fn main() {
    let mut input_name = String::new();
    let mut input_year = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    io::stdin().read_line(&mut input_year).unwrap();
    let name = input_name.trim();
    let year: i32 = input_year.trim().parse().unwrap();

    // Write your code below
    
    
    let secret_code = format!("⭐{}⭐-{}", name.chars().nth(0).unwrap().to_uppercase(), year.to_string().chars().rev().collect::<String>());
    println!("{}", secret_code);
}
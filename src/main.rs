use std::io;

fn convert_and_print(s: &str, n: f64, to_string: bool) {
    // Write your code here
    if to_string {
        let n_str = n.to_string().replace(".", "").replace("-", "");
        println!("Number: {}, Digits: {}", n, n_str.len());
    } else {
        let float: f64 = s.parse().unwrap();
        let int: i32 = float as i32;
        println!("String as number: {}", int);
    }
}

fn main() {
    let mut input_number_str = String::new();
    let mut input_n = String::new();

    io::stdin().read_line(&mut input_number_str).unwrap();
    io::stdin().read_line(&mut input_n).unwrap();

    let n: f64 = input_n.trim().parse().unwrap();
    let number_str = input_number_str.trim();

    // Call convert_and_print with to_string = false
    convert_and_print(number_str, n, false);

    // Call convert_and_print with to_string = true
    convert_and_print(number_str, n, true);
}

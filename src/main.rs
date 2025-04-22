use std::io;

fn bigger(arg1: f64, arg2: f64) -> f64 {
    // Complete the function
    if arg1 > arg2 {
        return arg1;
    } else {
        return arg2;
    }
}

fn main() {
    let mut input_iter = String::new();
    let mut input_num1 = String::new();
    let mut input_num2 = String::new();

    io::stdin().read_line(&mut input_iter).unwrap();
    io::stdin().read_line(&mut input_num1).unwrap();
    io::stdin().read_line(&mut input_num2).unwrap();

    let iter: i32 = input_iter.trim().parse().unwrap();
    let mut num1: f64 = input_num1.trim().parse().unwrap();
    let mut num2: f64 = input_num2.trim().parse().unwrap();

    for _ in 0..iter {
        // Write your code below
        let mut ret: f64 = bigger(num1, num2);
        if ret == num1 {
            ret /= 2.0;
            num1 = ret;
        } else {
            ret /= 2.0;
            num2 = ret;
        }
        println!("{}", ret);
        if ret <= 2.0 {
            break;
        }
    }
}

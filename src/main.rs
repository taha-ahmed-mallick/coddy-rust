fn get_user_info() -> (String, i32, String) {
    // Create name, age, and email variables here
    let name: String = String::from("Bob");
    let age: i32 = 25;
    let email: String = "bob@example.com".to_string();
    // Return the tuple
    return (name, age, email);
}

fn main() {
    // Call get_user_info and destructure the returned tuple
    let (name, age, email) = get_user_info();
    
    // Print the values
    println!("Name: {}, Age: {}, Email: {}", name, age, email);
}

/// This function is to demonstrate the printing of the String object
fn print_string(input_string: String) {
    println!("{}", input_string);
}

/// This function is to demonstrate
fn print_str(input_str: &str) {
    println!("{}", input_str);
}

/// This is the entry point for running the code
fn main() {
    let test_string = String::from("Hello, World!");
    let test_str = &"Hello, World!";
    print_string(test_string);
    print_str(test_str);
}

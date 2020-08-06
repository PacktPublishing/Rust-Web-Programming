

fn main() {
    // defines an immutable array
    let int_array: [i32; 3] = [1, 2, 3];

    // loops through and prints elements of the array
    for i in int_array.iter() {
        println!("{}", i);
    }

    // defines a vector of strings
    let str_vector: Vec<&str> = vec!["one", "two", "three"];

    // loops through and prints elements of the vector
    for i in str_vector.iter() {
        println!("{}", i);
    }

    let second_int_array: [i32; 3] = [1, 2, 3];

    // accesses an element in array
    let two = second_int_array[1];
    println!("{}", two);

    // define mutable string vector
    let mut str_vector: Vec<&str> = vec!["one", "two", "three"];

    // append to vector
    str_vector.push("four");

    for i in str_vector.iter() {
        println!("{}", i);
    }
}

use std::collections::HashMap;


fn main() {
    // define a mutable hashmap with strings as keys and i8 integers as values
    let mut general_map: HashMap<&str, i8> = HashMap::new();

    // insert a new entry into hash map
    general_map.insert("test", 25);

    // get the value from the hashmap
    let outcome: Option<&i8> = general_map.get("test");
    println!("{}", outcome.unwrap());

    // matches an action based on the result
    match general_map.get("test") {
        None => println!("it failed"),
        Some(result) => println!("Here is the result: {}", result)
    }

    // nested match => if getting "testing" fails gets "test"
    match general_map.get("testing") {
        None => {
            match general_map.get("test") {
                None => println!("Both testing and test failed"),
                Some(result) => println!("testing failed but test is: {}", result)
            }
        },
        Some(result) => println!("Here is the result: {}", result)
    }

}

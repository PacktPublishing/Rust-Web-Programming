use std::collections::HashMap;

// defines the human struct
struct Human {
    name: String,
    age: i8,
    current_thought: String
}

// defines functions for the Human struct
impl Human {

    // the constructor function for the Human struct
    fn new(input_name: String, input_age: i8) -> Human {
        return Human {
            name: input_name,
            age: input_age,
            current_thought: String::from("nothing")
        }
    }

    // this function enables the optional thought attribute to be added if needed
    fn with_thought(mut self, thought: String) -> Human {
        self.current_thought = String::from(thought);
        return self
    }

    // this function uses attributes from the struct to perform a task
    fn speak(&self) -> () {
        println!("Hello my name is {} and I'm {} years old.", &self.name, &self.age);
    }
}

// defines the enum to enable multiple data types
enum AllowedData {
    S(String),
    I(i8)
}

// defines a custom hash map that accepts all types belonging to the AllowedData enum
struct CustomMap {
    body: HashMap<String, AllowedData>
}

impl CustomMap {

    // the constructor for the CustomMap
    fn new() -> CustomMap {
        let custom_map: HashMap<String, AllowedData> = HashMap::new();
        return CustomMap{body: custom_map}
    }

    // gets data that can be either type defined in the enum
    fn get(&self, key: String) -> &AllowedData {
        return self.body.get(key.as_str()).unwrap()
    }

    // inserts data into the hashmap
    fn insert(&mut self, key: String, value: AllowedData) -> () {
        self.body.insert(key, value);
    }

    // displays
    fn display(&self, key: String) -> () {
        match self.get(key) {
            AllowedData::I(value) => println!("{}", value),
            AllowedData::S(value) => println!("{}", value)
        }
    }
}



fn main() {
    // creates a basic struct
    let developer = Human::new(String::from("Maxwell Flitton"), 31);
    developer.speak();
    println!("currently I'm thinking {}", developer.current_thought);

    // creates a basic struct with additional thought
    let new_developer = Human::new(String::from("Britta"), 28).with_thought(
        String::from("I'm Hungry"));
    new_developer.speak();
    println!("currently I'm thinking {}", new_developer.current_thought);

    // defining a new hash map
    let mut map = CustomMap::new();

    // inserting two different types of data
    map.insert(String::from("test"), AllowedData::I(8));
    map.insert(String::from("testing"), AllowedData::S(String::from("test value")));

    // displaying the data
    map.display(String::from("test"));
    map.display(String::from("testing"));
}

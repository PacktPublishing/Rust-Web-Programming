mod state;
mod to_do;

use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;


fn main() {

    // collects the environment arguments passed in by the user and collects to a vector of strings
    let args: Vec<String> = env::args().collect();

    // defines the commands from the environment
    let status: &String = &args[1];
    let title: &String = &args[2];

    // read the json file and print it using the debug notation
    let mut state: Map<String, Value> = read_file(String::from("./state.json"));
    println!("{:?}", state);

    // insert the title and status into the map
    state.insert(title.to_string(), json!(status));

    // write the map to the JSON file
    write_to_file(String::from("./state.json"), &mut state);

    // creates a pending item called washing
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory(
        String::from("pending"), String::from("washing")
    );

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("it's a done item with the title: {}", item.super_struct.title)
    }
}

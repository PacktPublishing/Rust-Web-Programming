use std::option::Option;
use serde_json::Map;
use serde_json::value::Value;


/// Trait for getting to do items.
pub trait Get {

    /// Gets a to do item and prints it out to the console.
    ///
    /// # Arguments
    /// * title (&String): the title of the to do item being fetched
    /// * state (&serde_json::value::Value): The loaded values from the state file
    ///
    /// # Returns
    /// None
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("item: {} was not found", title)
        }
    }
}

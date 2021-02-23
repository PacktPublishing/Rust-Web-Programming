use std::vec::Vec;

use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};
use crate::json_serialization::to_do_items::ToDoItems;


/// Gets all the to do items from the state JSON file and processes them to be serialized.
///
/// # Arguments
/// None
///
/// # Returns
/// * (ToDoItems): to do items sorted into Done and Pending with count numbers
pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file(String::from(
        "./state.json"));

    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let item_type: String = String::from(value.as_str().unwrap());

        let item: ItemTypes = to_do_factory(&item_type, key).unwrap();
        array_buffer.push(item);
    }
    return ToDoItems::new(array_buffer);
}

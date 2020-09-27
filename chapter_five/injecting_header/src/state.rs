use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;


/// This function reads a JSON file from disk.
///
/// # Arguments
/// * file_name (String): the path to the file being read
///
/// # Returns
/// (serde_json::value::Value): the values from the JSON file
pub fn read_file(file_name: String) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state
}

/// This function writes a JSON map to file on disk.
///
/// # Arguments
/// * file_name (String): the path to the file being read
/// * state (Map<String, Value>): the data being written to disk
///
/// # Returns
/// None
pub fn write_to_file(file_name: String, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}

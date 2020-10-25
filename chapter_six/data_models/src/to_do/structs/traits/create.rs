use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;


/// Trait for creating to do items.
pub trait Create {

    /// Creates a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title for the to item to be created
    ///
    /// # Returns
    /// None
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being created\n\n", title);
    }
}

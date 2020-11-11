use serde_json::Map;
use serde_json::value::Value;

use crate::state::write_to_file;


/// Trait for deleting to do items.
pub trait Delete {

    /// Deletes a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title of the item to be deleted
    ///
    /// # Returns
    /// None
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}

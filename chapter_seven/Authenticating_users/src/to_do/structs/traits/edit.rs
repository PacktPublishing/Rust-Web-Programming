use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;

/// The trait for editing steps.
pub trait Edit {

    /// Sets the to do item to Done.
    ///
    /// # Arguments
    /// * title (String): title of the to do item being edited
    ///
    /// # Returns
    /// None
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    /// Sets the to do item to Pending.
    ///
    /// # Arguments
    /// * title (String): title of the to do item being edited
    ///
    /// # Returns
    /// None
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file(String::from("./state.json"), state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}

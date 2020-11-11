use serde::Serialize;


/// This struct defines the key attributes for a to do struct.
///
/// # Attributes
/// * title (String): the title of the to do item
/// * status (String): the status of the to do item
#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String
}

impl Base {

    /// The constructor for the Base struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    /// * status (String): the status of the to do item
    ///
    /// # Returns
    /// (Base): the constructed Base struct
    pub fn new(input_title: String, input_status: String) -> Base {
        return Base {title: input_title, status: input_status}
    }
}

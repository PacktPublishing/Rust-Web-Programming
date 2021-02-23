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


#[cfg(test)]
mod base_tests {

    use super::Base;

    #[test]
    fn new() {
        let title: String = String::from("test title");
        let expected_title: String = String::from("test title");
        let status: String = String::from("test status");
        let expected_status: String = String::from("test status");

        let new_base_struct: Base = Base::new(title, status);
        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }

}

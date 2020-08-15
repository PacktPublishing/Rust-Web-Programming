use super::base::Base;
use super::traits::to_do_item::ToDoItem;

/// This struct defines a to do item for a Pending to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Pending {
    pub super_struct: Base
}

impl Pending {

    /// The constructor for the Done struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    ///
    /// # Returns
    /// (Pending): the constructed Pending struct
    pub fn new(input_title: String) -> Pending {
        let input_status: String = String::from("pending");
        let base: Base = Base::new(input_title, input_status);
        return Pending{super_struct: base}
    }
}

impl ToDoItem for Pending {}

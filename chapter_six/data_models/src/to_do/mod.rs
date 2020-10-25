pub mod structs;
pub use structs::done::Done;
pub use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

/// This function builds and returns to do structs.
///
/// # Augments
/// * item_type (&String): the type of struct to be built and returned
/// * item_title (String): the title for the item to be built
///
/// # Returns
/// (Result<ItemTypes, &'templates str>):
pub fn to_do_factory(item_type: &String, item_title: String) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemTypes::Pending(pending_item))
    }
    else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemTypes::Done(done_item))
    }
    else {
        Err("this is not accepted")
    }
}

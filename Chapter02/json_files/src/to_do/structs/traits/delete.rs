

/// Trait for deleting to do items.
pub trait Delete {

    /// Deletes a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title of the item to be deleted
    ///
    /// # Returns
    /// None
    fn delete(&self, title: String) {
        println!("{} is being deleted", title);
    }
}

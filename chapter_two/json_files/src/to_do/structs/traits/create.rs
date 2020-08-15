

/// Trait for creating to do items.
pub trait Create {

    /// Creates a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title for the to item to be created
    ///
    /// # Returns
    /// None
    fn create(&self, title: &String) {
        println!("{} is being created", title);
    }
}
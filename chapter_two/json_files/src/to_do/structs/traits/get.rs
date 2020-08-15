

/// Trait for getting to do items.
pub trait Get {

    /// Gets a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title of the to do item being fetched
    ///
    /// # Returns
    /// None
    fn get(&self, title: &String) {
        println!("{} is being fetched", title);
    }
}

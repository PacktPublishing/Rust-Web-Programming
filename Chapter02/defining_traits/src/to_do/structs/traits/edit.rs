

/// The trait for editing steps.
pub trait Edit {

    /// Sets the to do item to Done.
    ///
    /// # Arguments
    /// * title (String): title of the to do item being edited
    ///
    /// # Returns
    /// None
    fn set_to_done(&self, title: &String) {
        println!("{} is being set to done", title);
    }

    /// Sets the to do item to Pending.
    ///
    /// # Arguments
    /// * title (String): title of the to do item being edited
    ///
    /// # Returns
    /// None
    fn set_to_pending(&self, title: &String) {
        println!("{} is being set to done", title);
    }
}

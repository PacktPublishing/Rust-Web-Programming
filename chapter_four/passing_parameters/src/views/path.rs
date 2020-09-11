

/// This struct defines the Path for a App route.
///
/// # Attributes
/// * prefix (String): the prefix of the view
pub struct Path {
    pub prefix: String
}

impl Path {

    /// This function defines a full path based on the struct's prefix and the String passed in.
    ///
    /// # Arguments
    /// * following_path (String): the rest of the path to be appended to the self.prefix
    ///
    /// # Use
    /// To use this in a route, we have to reference it:
    ///
    /// ```rust
    /// let path = Path{base: String::from("/base/")};
    /// app.route(&path.define(String::from("tail/path")), web::get().to(login::login))
    /// ```
    pub fn define(&self, following_path: String) -> String {
        return self.prefix.to_owned() + &following_path
    }
}

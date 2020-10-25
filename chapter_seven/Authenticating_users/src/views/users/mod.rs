use actix_web::web;
mod create;
use super::path::Path;


/// This function adds the user views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn user_factory(app:  &mut web::ServiceConfig) {
    let base_path: Path = Path{prefix: String::from("/user")};

    app.route(&base_path.define(String::from("/create")),
              web::post().to(create::create));
}

use actix_web::web;
mod content_loader;
mod items;
mod login;
mod logout;
use super::path::Path;


/// This function adds the app views to the web server serving HTML.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn app_factory(app: &mut web::ServiceConfig) {
    // define the path struct
    let base_path: Path = Path{prefix: String::from("/")};
    // define the routes for the app
    app.route(&base_path.define(String::from("")),
              web::get().to(items::items));
    app.route(&base_path.define(String::from("login/")),
              web::get().to(login::login));
    app.route(&base_path.define(String::from("logout/")),
              web::get().to(logout::logout));
}

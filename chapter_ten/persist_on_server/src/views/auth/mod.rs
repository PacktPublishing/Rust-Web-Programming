use actix_web::web;
mod login;
mod logout;
use super::path::Path;


/// This function adds the auth views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn auth_factory(app: &mut web::ServiceConfig) {
    // define the path struct
    let base_path: Path = Path{prefix: String::from("/auth"), backend: true};
    // define the routes for the app
    let app = app.route(&base_path.define(String::from("/login")),
                        web::post().to(login::login));
    app.route(&base_path.define(String::from("logout/")),
              web::get().to(logout::logout));
}
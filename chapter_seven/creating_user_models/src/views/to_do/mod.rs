use actix_web::web;
mod utils;
mod create;
mod get;
mod edit;
mod delete;
use super::path::Path;


/// This function adds the to do item views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn item_factory(app: &mut web::ServiceConfig) {
    // define the path struct
    let base_path: Path = Path{prefix: String::from("/item")};
    // define the routes for the app
   app.route(&base_path.define(String::from("/create/{title}")),
             web::post().to(create::create));

    app.route(&base_path.define(String::from("/get")),
              web::get().to(get::get));

    app.route(&base_path.define(String::from("/edit")),
              web::put().to(edit::edit));

    app.route(&base_path.define(String::from("/delete")),
              web::post().to(delete::delete));
}

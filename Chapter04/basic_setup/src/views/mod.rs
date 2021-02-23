use actix_web::web;
mod path;
mod auth;
use std::env;


/// This function combines the views from other view modules.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn views_factory(app: &mut web::ServiceConfig) {

    let args: Vec<String> = env::args().collect();
    let param: &String = &args[args.len() - 1];

    if param.as_str() == "cancel_logout" {
        println!("logout view isn't being configured");
        auth::auth_factory(app, false);
    } else {
        println!("logout view is being configured");
        auth::auth_factory(app, true);
    }
}

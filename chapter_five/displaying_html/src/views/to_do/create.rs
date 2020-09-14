use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;
use actix_web::Responder;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;
use super::utils::return_state;


/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (impl Responder ): message to be sent back to the user
pub async fn create(req: HttpRequest) -> impl Responder {

    // load the data from the state JSON file
    let state: Map<String, Value> = read_file(String::from(
        "./state.json"));

    // get the title from the http request
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();

    // create the to do item
    let item = to_do::to_do_factory(&String::from("pending"),
                                    title).expect("create ");

    // add the to do item from the state.json
    process_input(item, "create".to_string(), &state);

    // return results
    return return_state()
}

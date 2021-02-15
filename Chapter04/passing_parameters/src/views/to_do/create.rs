use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;


/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (String): message to be sent back to the user
pub async fn create(req: HttpRequest) -> String {

    // load the data from the state JSON file
    let state: Map<String, Value> = read_file(String::from(
        "./state.json"));

    // get the title from the http request
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();
    let title_reference: String = title.clone();

    // create the to do item
    let item = to_do::to_do_factory(&String::from("pending"),
                                    title).expect("create ");

    // add the to do item from the state.json
    process_input(item, "create".to_string(), &state);

    // return a message to viewer
    return format!("{} created", title_reference)
}

use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;
use crate::state::read_file;

use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;


/// This function edits a to do item's status.
///
/// # Arguments
/// * to_di_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {

    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title_reference: &String = &to_do_item.title.clone();
    let title: String = to_do_item.title.clone();

    let status: String;
    match &state.get(title_reference) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None=> {
            return HttpResponse::NotFound().json(
                format!("{} not in state", title_reference))
        }
    }

    if &status == &to_do_item.status {
        return HttpResponse::Ok().json(return_state())
    }

    match to_do_factory(&status, title) {
        Err(_item) => return HttpResponse::BadRequest().json(
            format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("edit"), &state)
    }
    return HttpResponse::Ok().json(return_state())
}

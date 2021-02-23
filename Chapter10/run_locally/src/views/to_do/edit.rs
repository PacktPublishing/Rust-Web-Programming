use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::auth::jwt::JwtToken;
use crate::schema::to_do;


/// This function edits a to do item's status.
///
/// # Arguments
/// * to_do_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
/// * req (HttpRequest): The request being made
///
/// # Returns
/// * (HttpResponse): response body to be passed to the viewer.
pub async fn edit(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {

    let title_ref: String = to_do_item.title.clone();
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title
        .eq(title_ref))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    return HttpResponse::Ok().json(return_state(&token.user_id))
}

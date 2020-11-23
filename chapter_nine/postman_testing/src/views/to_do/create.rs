use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpRequest;
use actix_web::Responder;

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::auth::jwt::JwtToken;
use crate::schema::to_do;
use super::utils::return_state;


/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (impl Responder): message to be sent back to the user
pub async fn create(req: HttpRequest) -> impl Responder {
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();
    let title_ref: String = req.match_info().get("title"
    ).unwrap().to_string();

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id.clone());
        let _ = diesel::insert_into(to_do::table).values(&new_post)
            .execute(&connection);
    }

    return return_state(&token.user_id)
}

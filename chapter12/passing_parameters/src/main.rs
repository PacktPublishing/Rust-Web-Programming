use warp::Filter;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
use diesel::prelude::*;
extern crate dotenv;

mod our_jwt;
mod schema;
mod to_do;
mod json_serialization;
mod database;
mod models;

use std::vec::Vec;

use to_do::to_do_factory;
use database::establish_connection;
use models::item::item::Item;
use json_serialization::to_do_items::ToDoItems;
use json_serialization::to_do_item::ToDoItem;
use crate::models::item::new_item::NewItem;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let home = warp::path!("home")
        .map(|| "This is a Warp server built in Rust");

    let greet = warp::path!("greet" / String / i32)
        .map(|name: String, age: i32| {
           return format!("I am {} and {} years old", name, age)
        });

    let add = warp::path!("add" / i32 / i32)
        .map(|one: i32, two: i32| {
            let result: i32 = one + two;
            return warp::reply::json(&result)
        });

    async fn get_items_reply(token: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        match our_jwt::JwtToken::decode(token) {
            Ok(token) => {
                let connection = establish_connection();

                let items = schema::to_do::table
                    .order(schema::to_do::columns::id.asc())
                    .filter(schema::to_do::columns::user_id.eq(&token.user_id))
                    .load::<Item>(&connection)
                    .unwrap();

                let mut array_buffer = Vec::new();

                for item in items {
                    let item = to_do_factory(&item.status, item.title).unwrap();
                    array_buffer.push(item);
                }
                return Ok(Box::new(warp::reply::json(&ToDoItems::new(array_buffer))))
            },
            Err(_message) => {
                Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
            }
        }
    }

    let get_items = warp::path!("items")
        .and(warp::header("user-token")).and_then(get_items_reply);

    async fn make_item_reply(token: String, item: ToDoItem) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        match our_jwt::JwtToken::decode(token) {
            Ok(token) => {
                let connection = establish_connection();

                let title: String = item.title.clone();
                let title_ref: String = item.title.clone();


                let items = schema::to_do::table
                    .filter(schema::to_do::columns::title.eq(title_ref.as_str()))
                    .filter(schema::to_do::columns::user_id.eq(&token.user_id))
                    .order(schema::to_do::columns::id.asc())
                    .load::<Item>(&connection)
                    .unwrap();

                if items.len() == 0 {
                    let new_post = NewItem::new(title, token.user_id.clone());
                    let _ = diesel::insert_into(schema::to_do::table).values(&new_post)
                        .execute(&connection);
                }

                let items = schema::to_do::table
                    .order(schema::to_do::columns::id.asc())
                    .filter(schema::to_do::columns::user_id.eq(&token.user_id))
                    .load::<Item>(&connection)
                    .unwrap();

                let mut array_buffer = Vec::new();

                for item in items {
                    let item = to_do_factory(&item.status, item.title).unwrap();
                    array_buffer.push(item);
                }
                return Ok(Box::new(warp::reply::json(&ToDoItems::new(array_buffer))))
            },
            Err(_message) => {
                return Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
            }
        }
    }

    let make_item = warp::post()
        .and(warp::path("make"))
        .and(warp::header("user-token"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(make_item_reply);


    let log = warp::log("to_do::api");

    let routes = warp::get().and(
            home
            .or(greet)
            .or(add)
            .or(get_items)
    ).or(make_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

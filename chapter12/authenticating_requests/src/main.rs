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

    let log = warp::log("to_do::api");

    let routes = warp::get().and(
            home
            .or(greet)
            .or(add)
            .or(get_items)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

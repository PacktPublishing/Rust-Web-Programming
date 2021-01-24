use warp::Filter;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
extern crate dotenv;

mod to_do;
mod json_serialization;
mod database;
mod models;
mod schema;

// use crate::diesel;
// use diesel::prelude::*;

// use std::vec::Vec;
//
// use to_do::to_do_factory;
// use json_serialization::to_do_items::ToDoItems;
//
// use database::establish_connection;
// use models::item::item::Item;
// use schema::to_do;
//
//
// /// Gets all the to do items from the state JSON file and processes them to be serialized.
// ///
// /// # Arguments
// /// user_id (&i32): the user id belonging to the request
// ///
// /// # Returns
// /// * (ToDoItems): to do items sorted into Done and Pending with count numbers
// pub fn return_state(user_id: &i32) -> ToDoItems {
//     let connection = establish_connection();
//
//     let items = to_do::table
//         .order(to_do::columns::id.asc())
//         .filter(to_do::columns::user_id.eq(&user_id))
//         .load::<Item>(&connection)
//         .unwrap();
//
//     let mut array_buffer = Vec::new();
//
//     for item in items {
//         let item = to_do_factory(&item.status, item.title).unwrap();
//         array_buffer.push(item);
//     }
//     return ToDoItems::new(array_buffer);
// }


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

    let log = warp::log("to_do::api");

    let routes = warp::get().and(
            home
            .or(greet)
            .or(add)
    ).with(log);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
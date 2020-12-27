#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use rocket_contrib::json::Json;
use diesel::prelude::*;

mod jwt;
mod to_do;
mod schema;
mod database;
mod models;
mod json_serialization;

use crate::database::DbConn;
use crate::models::item::item::Item;
use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age, name)
}

#[get("/get")]
fn get_items(conn: DbConn, token: jwt::JwtToken) -> Json<ToDoItems> {

    let items = schema::to_do::table
        .order(schema::to_do::columns::id.asc())
        .filter(schema::to_do::columns::user_id.eq(&token.user_id))
        .load::<Item>(&*conn)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let item = to_do_factory(&item.status, item.title).unwrap();
        array_buffer.push(item);
    }
    return Json(ToDoItems::new(array_buffer))
}


fn main() {
    rocket::ignite()
        .mount("/", routes![hello, bye])
        .mount("/items", routes![get_items])
        .attach(DbConn::fairing())
        .launch();
}

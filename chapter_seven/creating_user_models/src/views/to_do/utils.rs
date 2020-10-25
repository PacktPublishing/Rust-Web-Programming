use crate::diesel;
use diesel::prelude::*;

use std::vec::Vec;

use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;


/// Gets all the to do items from the state JSON file and processes them to be serialized.
///
/// # Arguments
/// None
///
/// # Returns
/// * (ToDoItems): to do items sorted into Done and Pending with count numbers
pub fn return_state() -> ToDoItems {
    let connection = establish_connection();

    let items = to_do::table
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let item = to_do_factory(&item.status, item.title).unwrap();
        array_buffer.push(item);
    }
    return ToDoItems::new(array_buffer);
}

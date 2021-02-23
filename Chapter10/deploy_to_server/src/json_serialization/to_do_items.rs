use std::vec::Vec;

use actix_web::{Responder, Error, HttpResponse, HttpRequest};
use serde::Serialize;
use futures::future::{ready, Ready};

use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;


/// This struct packages the raw struct fields to package items for JSON serialization.
///
/// # Parameters
/// * pending_items (Vec<Base>): vector containing the statuses and titles of pending items
/// * done_items (Vec<Base>): vector containing the statuses and titles of the done items
/// * pending_item_count (i8): the number of pending items
/// * done_item_count (i8): the number of done items
#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}

impl ToDoItems {

    /// This function constructs the ToDoItems struct.
    ///
    /// # Arguments
    /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
    ///
    /// # Returns
    /// * (ToDoItems): package struct
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(
                    packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(
                    packed.super_struct)
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;
        return ToDoItems{
            pending_items: pending_array_buffer, done_item_count: done_count,
            pending_item_count: pending_count, done_items: done_array_buffer
        }
    }
}

impl Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    /// This function gets fired when the struct is being returned in an actix view.
    ///
    /// # Arguments
    /// * _req (&HttpRequest): the request belonging to the view
    ///
    /// # Returns
    /// * (Self::Future): a OK HTTP response with the serialized struct in the body
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

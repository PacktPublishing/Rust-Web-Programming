use serde::Deserialize;


#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}

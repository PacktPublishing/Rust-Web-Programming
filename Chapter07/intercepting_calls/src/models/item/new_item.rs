use crate::schema::to_do;


#[derive(Insertable)]
#[table_name="to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
}

impl NewItem {
    pub fn new(title: String) -> NewItem {
        return NewItem{title, status: String::from("pending")}
    }
}
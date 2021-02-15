use crate::schema::to_do;


#[derive(Queryable, Identifiable)]
#[table_name="to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
}

impl Item {

}

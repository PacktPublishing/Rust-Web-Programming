use serde::Deserialize;


#[derive(Deserialize)]
pub struct NewUserSchema {
    pub name: String,
    pub email: String,
    pub password: String
}

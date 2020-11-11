use serde::Deserialize;


#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}
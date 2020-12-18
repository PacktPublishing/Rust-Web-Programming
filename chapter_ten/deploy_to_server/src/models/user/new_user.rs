extern crate bcrypt;

use uuid::Uuid;
use diesel::Insertable;
use bcrypt::{DEFAULT_COST, hash};

use crate::schema::users;


#[derive(Insertable, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {

    /// Creates a new instance of the NewUser struct.
    ///
    /// # Arguments
    /// * name (String): The name of the user
    /// * username (String): The username of the user
    /// * email (String): The email associated  with the user
    /// * password (String): The password for the user
    ///
    /// # Returns
    /// (NewUser):
    pub fn new(name: String, email: String, password: String) -> NewUser {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        return NewUser {name, email, password: hashed_password, unique_id: uuid}
    }
}
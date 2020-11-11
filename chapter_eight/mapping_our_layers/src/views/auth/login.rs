use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::auth::jwt::JwtToken;


pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let connection = establish_connection();
    let users = users::table
        .filter(users::columns::name.eq(username.as_str()))
        .load::<User>(&connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap()
    }

    match users[0].clone().verify(password) {
        true => {
            let token: String = JwtToken::encode(
                users[0].clone().id);
            HttpResponse::Ok().header(
                "token", token).await.unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}
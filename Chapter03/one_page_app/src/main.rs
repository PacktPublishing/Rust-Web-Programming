use actix_web::{web, App, HttpRequest, HttpServer, Responder};


/// This function defines a logout view.
///
/// # Arguments
/// None
///
/// # Returns
/// (String) message stating that it's the logout view
pub async fn logout() -> String {
    format!("Logout view")
}


/// This function defines a login view.
///
/// # Arguments
/// None
///
/// # Returns
/// (String) message stating that it's the login view
pub async fn login() -> String {
    format!("Login view")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("function is firing");
        let app = App::new()
            .route("/auth/login", web::get().to(login))
            .route("/auth/logout", web::get().to(logout));
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
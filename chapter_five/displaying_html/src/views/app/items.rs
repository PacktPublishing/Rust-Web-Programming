use actix_web::HttpResponse;


pub async fn items() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Items</h1>")
}
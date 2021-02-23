use actix_web::HttpResponse;
use super::content_loader::read_file;


/// Renders the main view that shows all items in the state.
///
/// # Arguments
/// *
///
/// # Returns
/// * (HttpResponse) with HTML
pub async fn items() -> HttpResponse {
    let html_data = read_file(
        String::from("./templates/main.html"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}

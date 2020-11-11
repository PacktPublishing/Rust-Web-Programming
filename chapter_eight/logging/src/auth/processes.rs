use actix_web::dev::ServiceRequest;
use super::jwt;


/// Checks to see if the token matches.
///
/// # Parameters
/// * password (String): password to be checked
///
/// # Returns
/// * (Result<String, &'templates str>): password if correct, error message if not
pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(message) => Err(message)
    }
}


/// Extracts the header from the request.
///
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function
///
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, error message if not
pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {

    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_password) => Ok(String::from(processed_password)),
                Err(_processed_password) => Err("there was an error processing token")
            }
        },
        None => Err("there is no token")
    }
}

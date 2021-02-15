use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;


/// Processes the token to see if the correct token is in the header.
///
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function
///
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, error message if not
pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => {
            match processes::check_password(token) {
                Ok(token) => Ok(token),
                Err(message) => Err(message)
            }
        },
        Err(message) => Err(message)
    }
}
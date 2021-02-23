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


#[cfg(test)]
mod process_token_tests {

    use super::process_token;
    use super::jwt::JwtToken;
    use actix_web::test::TestRequest;

    #[test]
    fn no_token_process_token() {
        let mock_request = TestRequest::with_header("test", "test").to_srv_request();

        match process_token(&mock_request) {
            Err(message) => assert_eq!("there is no token", message),
            _ => panic!("No token in request header should fail")
        }
    }

    #[test]
    fn incorrect_token() {
        let mock_request = TestRequest::with_header("user-token", "test").to_srv_request();

        match process_token(&mock_request) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("Incorrect token should error")
        }
    }

    #[test]
    fn correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let mock_request = TestRequest::with_header("user-token", encoded_token).to_srv_request();

        match process_token(&mock_request) {
            Ok(token) => assert_eq!("passed", token),
            _ => panic!("encoded token should pass")
        }
    }

}

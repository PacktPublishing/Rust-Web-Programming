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


#[cfg(test)]
mod check_credentials_tests {

    use super::super::jwt::JwtToken;
    use super::extract_header_token;
    use super::check_password;
    use actix_web::test;

    #[test]
    fn correct_check_password() {
        let token: String = JwtToken::encode(32);

        match check_password(token) {
            Ok(message) => assert_eq!(String::from("passed"), message),
            _ => panic!("correct password should pass")
        }
    }

    #[test]
    fn incorrect_check_password() {
        let password: String = String::from("test");

        match check_password(password) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("check password should not be able to be decoded")
        }
    }

    #[test]
    fn no_token_in_extract_header_token() {
        let mock_request = test::TestRequest::with_header("test", "test").to_srv_request();

        match extract_header_token(&mock_request) {
            Err(message) => assert_eq!("there is no token", message),
            _ => panic!("token should not be present in service request")
        }
    }

    #[test]
    fn correct_token_in_extract_header_token() {
        let mock_request = test::TestRequest::with_header("user-token", "test").to_srv_request();

        match extract_header_token(&mock_request) {
            Ok(token) => assert_eq!(String::from("test"), token),
            _ => panic!("token should be present in the header")
        }
    }

}

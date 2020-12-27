use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::result::Result;


#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}


/// Holds parameters that were encoded in the JSON web token.
///
/// # Attributes
/// * user_id (i32): the ID of a user
pub struct JwtToken {
    pub user_id: i32,
    pub body: String
}

impl JwtToken {

    /// Creates a JSON web token.
    ///
    /// # Arguments
    /// * user_id (i32): ID of the user to be encoded into the token.
    ///
    /// # Returns
    /// (String): The token with all arguments encoded into it
    pub fn encode(user_id: i32) -> String {
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(&secret_key.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str = claims.sign_with_key(&key).unwrap();
        return String::from(token_str)
    }

    /// Extracts the user ID from the encoded JSON web token.
    ///
    /// # Arguments
    /// * encoded_token (String): The JSON web token to be decoded.
    ///
    /// # Returns
    /// (JwtToken): struct containing parameters from the token
    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(&secret_key.as_bytes()).unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, jwt::Error> = VerifyWithKey::verify_with_key(token_str, &key);

        match token {
            Ok(token) => Ok(JwtToken { user_id: token.claims()["user_id"], body: encoded_token}),
            Err(_) => Err("could not decode token")
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JwtToken {
    type Error = ApiKeyError;

    /// Fires when the request is processed as a request guard extracting the JWT token and decoding
    /// it.
    ///
    /// # Arguments
    /// * request (&'a Request<'r>): the rocket request containing the header
    ///
    /// # Returns
    /// * (request::Outcome<Self, Self::Error>) error if failed, JwtToken if not
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("user-token").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::NotFound, ApiKeyError::Missing)),
            1 => {
                let token = JwtToken::decode(String::from(keys[0].to_string()));

                match token {
                    Ok(token) => Outcome::Success(token),
                    Err(_message) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
            },
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
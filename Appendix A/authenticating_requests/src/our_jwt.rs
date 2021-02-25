extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;


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
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str: String = claims.sign_with_key(&key).unwrap();
        return token_str
    }

    /// Extracts the user ID from the encoded JSON web token.
    ///
    /// # Arguments
    /// * encoded_token (String): The JSON web token to be decoded.
    ///
    /// # Returns
    /// (Result<JwtToken, &'static str>): struct containing parameters from the token
    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> = VerifyWithKey::verify_with_key(token_str, &key);
        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                return Ok(JwtToken { user_id: claims["user_id"], body: encoded_token})
            }
            Err(_) => return Err("Could not decode")
        }
    }
}

#[cfg(test)]
mod jwt_tests {

    use super::JwtToken;
    use actix_web::test;

    #[test]
    fn encode_decode() {
        let encoded_token: String = JwtToken::encode(32);
        let decoded_token: JwtToken = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(32, decoded_token.user_id);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("test");

        match JwtToken::decode(encoded_token) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("Incorrect token should not be able to be encoded")
        }
    }

    #[test]
    fn decode_from_request_with_correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let request = test::TestRequest::with_header("user-token", encoded_token).to_http_request();
        let out_come = JwtToken::decode_from_request(request);

        match out_come {
            Ok(token) => assert_eq!(32, token.user_id),
            _ => panic!("Token is not returned when it should be")
        }

    }
}

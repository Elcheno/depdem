use crate::models::jwt_model::Claims;

use jsonwebtoken::errors::Error;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

pub fn create_jwt(id: i32, private_key: &[u8]) -> Result<String, Error> {
    let expiration = get_current_timestamp() + 604800;

    let claims = Claims {
        subject_id: id,
        exp: expiration,
    };

    let header = Header::new(Algorithm::RS256);

    let encoding_key = match EncodingKey::from_rsa_pem(private_key) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    encode(&header, &claims, &encoding_key)
}

pub fn decode_jwt(token: String, public_key: &[u8]) -> Result<Claims, Error> {
    let token = token.trim_start_matches("Bearer").trim();

    let dencoding_key = match DecodingKey::from_rsa_pem(public_key) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    match decode::<Claims>(&token, &dencoding_key, &Validation::new(Algorithm::RS256)) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err),
    }
}

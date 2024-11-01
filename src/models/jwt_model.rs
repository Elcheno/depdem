use crate::models::response_model::{NetworkResponse, Response, ResponseBody};
use crate::utils::jwt::decode_jwt;
// use crate::utils::keys::verify_signed_token;
use crate::models::path_model::PathModel;
use jsonwebtoken::errors::Error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    pub exp: u64,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            let public_key_file_path = PathModel::get_public_key_file_path();

            let public_key = match fs::read(public_key_file_path.unwrap()) {
                Ok(data) => data,
                Err(_) => Vec::new(),
            };
            Ok(decode_jwt(String::from(key), &public_key)?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    body: ResponseBody::Message(String::from(
                        "Error validating JWT token - No token provided",
                    )),
                };
                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(json!(response).to_string()),
                ))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Expired Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response).to_string()),
                        ))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Invalid Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response).to_string()),
                        ))
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - {}",
                                err
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response).to_string()),
                        ))
                    }
                },
            },
        }
    }
}

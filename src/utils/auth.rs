use crate::models::path_model::PathModel;
use crate::models::response_model::{NetworkResponse, ResponseBody};
use crate::utils::jwt::create_jwt;
use rocket::serde::json::json;
use std::fs;

pub fn auth() -> Result<String, NetworkResponse> {
    let private_key_file_path = PathModel::get_private_key_file_path();

    let private_key_data = match fs::read(private_key_file_path.unwrap()) {
        Ok(data) => data,
        Err(_) => {
            return Err(NetworkResponse::BadRequest(
                json!(ResponseBody::Message(
                    "Error to read private key".to_string()
                ))
                .to_string(),
            ))
        }
    };

    match create_jwt(5, &private_key_data) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}

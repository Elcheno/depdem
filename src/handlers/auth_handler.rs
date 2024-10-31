use crate::models::response_model::{NetworkResponse, Response, ResponseBody};
use crate::utils::auth::auth_user;
use crate::utils::keys::verify_keys;
use rocket::serde::json::{json, Value};

#[post("/login", format = "text", data = "<public_key>")]
pub async fn login(public_key: String) -> Result<Value, NetworkResponse> {
    match verify_keys(&public_key) {
        Ok(_) => {
            let token = auth_user()?;
            let response = Response {
                body: ResponseBody::AuthToken(token),
            };
            return Ok(json!(response));
        }
        Err(_) => {
            return Err(NetworkResponse::BadRequest(
                json!(ResponseBody::Message("Error to verify keys".to_string())).to_string(),
            ))
        }
    }
}

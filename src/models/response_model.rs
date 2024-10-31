use rocket::serde::Serialize;
use rocket::Responder;
use serde::Deserialize;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 500)]
    InternalServerError(String),
}

#[derive(Serialize, Deserialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
    Error(ResponseError),
    Success(ResponseSuccess),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResponseSuccess {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResponseError {
    pub out_code: Option<i32>,
    pub message: String,
    pub out_str: Option<String>,
}

impl ResponseSuccess {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn ok(message: String) -> Self {
        Self { code: 0, message }
    }
}

impl ResponseError {
    pub fn new(out_code: Option<i32>, message: String, out_str: Option<String>) -> Self {
        Self {
            out_code,
            message,
            out_str,
        }
    }
}

// Define tu error personalizado sin `thiserror`
// #[derive(Debug)]
// pub enum CustomError {
//     DatabaseError(diesel::result::Error),
// }

// // Implementa la conversión de `diesel::result::Error` a `CustomError`
// impl From<diesel::result::Error> for CustomError {
//     fn from(error: diesel::result::Error) -> Self {
//         CustomError::DatabaseError(error)
//     }
// }

// // Implementa la conversión de `CustomError` a `NetworkResponse`
// impl From<CustomError> for NetworkResponse {
//     fn from(error: CustomError) -> Self {
//         match error {
//             CustomError::DatabaseError(_) => {
//                 NetworkResponse::BadRequest("Database error".to_string())
//             }
//         }
//     }
// }

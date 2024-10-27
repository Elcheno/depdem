use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response as ResponseHTTP};
use rocket::serde::{Deserialize, Serialize};
use serde_json;
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResponseError {
    pub code: Status,
    pub out_code: Option<i32>,
    pub message: String,
    pub out_str: Option<String>,
}

impl Response {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn ok(message: String) -> Self {
        Self { code: 0, message }
    }
}

impl ResponseError {
    pub fn new(
        code: Status,
        out_code: Option<i32>,
        message: String,
        out_str: Option<String>,
    ) -> Self {
        Self {
            code,
            out_code,
            message,
            out_str,
        }
    }

    pub fn internal_server_error(
        out_code: Option<i32>,
        message: String,
        out_str: Option<String>,
    ) -> Self {
        Self {
            code: Status::InternalServerError,
            out_code,
            message,
            out_str,
        }
    }

    pub fn bad_request(out_code: Option<i32>, message: String, out_str: Option<String>) -> Self {
        Self {
            code: Status::BadRequest,
            out_code,
            message,
            out_str,
        }
    }
}

impl<'r> Responder<'r, 'static> for ResponseError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<ResponseHTTP<'static>, Status> {
        let body = serde_json::to_string(&self).unwrap();

        ResponseHTTP::build()
            .status(self.code)
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::JSON)
            .ok()
    }
}

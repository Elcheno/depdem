#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod models;
pub mod utils;

use dotenvy::dotenv;

use handlers::*;
use models::jwt_model::JWT;
use models::response_model::NetworkResponse;
use models::response_model::{ResponseBody, ResponseError, ResponseSuccess};
use rocket::serde::json::json;
use rocket::serde::json::Json;
use std::process::Command;
use utils::keys::{generate_keys, load_keys};
use utils::transform::transform_vec_to_string;

#[get("/")]
fn index() -> &'static str {
    "depdem running"
}

#[get("/service/status")]
fn service_status(
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;

    let output = Command::new("sh")
        .args(["-c", "service tomcat10 status"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(ResponseSuccess::ok("Service is running".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(NetworkResponse::InternalServerError(
        json!(ResponseBody::Error(ResponseError::new(
            Some(code),
            String::from("Error checking service status"),
            Some(std_err),
        )))
        .to_string(),
    ))
}

#[put("/service/stop")]
fn stop_service(
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;

    let output = Command::new("sh")
        .args(["-c", "service tomcat10 stop"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(ResponseSuccess::ok("Service stopped".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(NetworkResponse::InternalServerError(
        json!(ResponseBody::Error(ResponseError::new(
            Some(code),
            String::from("Error stopping service"),
            Some(std_err),
        )))
        .to_string(),
    ))
}

#[put("/service/start")]
fn start_service(
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;

    let output = Command::new("sh")
        .args(["-c", "service tomcat10 start"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(ResponseSuccess::ok("Service started".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(NetworkResponse::InternalServerError(
        json!(ResponseBody::Error(ResponseError::new(
            Some(code),
            String::from("Error to start service"),
            Some(std_err),
        )))
        .to_string(),
    ))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    match load_keys() {
        Ok(_) => println!("Keys loaded successfully"),
        Err(_) => generate_keys().expect("Error to generate keys"),
    }

    rocket::build().mount(
        "/depdem/api/v1",
        routes![
            index,
            stop_service,
            start_service,
            service_status,
            auth_handler::login
        ],
    )
}

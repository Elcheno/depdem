#[macro_use]
extern crate rocket;

pub mod models;
pub mod utils;

use models::response::{Response, ResponseError};
use rocket::serde::json::Json;
use std::process::Command;
use utils::transform::transform_vec_to_string;

#[get("/")]
fn index() -> &'static str {
    "depdem running"
}

#[get("/service/status")]
fn service_status() -> Result<Json<Response>, ResponseError> {
    let output = Command::new("sh")
        .args(["-c", "service tomcat status"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(Response::ok("Service is running".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(ResponseError::internal_server_error(
        Some(code),
        String::from("Error checking service status"),
        Some(std_err),
    ))
}

#[post("/service/stop")]
fn stop_service() -> Result<Json<Response>, ResponseError> {
    let output = Command::new("sh")
        .args(["-c", "service tomcat stop"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(Response::ok("Service stopped".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(ResponseError::internal_server_error(
        Some(code),
        String::from("Error stopping service"),
        Some(std_err),
    ))
}

#[post("/service/start")]
fn start_service() -> Result<Json<Response>, ResponseError> {
    let output = Command::new("sh")
        .args(["-c", "service tomcat start"])
        .output()
        .expect("failed to execute process");

    let code = output.status.code().unwrap();

    if code == 0 {
        return Ok(Json(Response::ok("Service started".to_string())));
    }

    let std_err: String = transform_vec_to_string(&output.stderr);

    Err(ResponseError::internal_server_error(
        Some(code),
        String::from("Error to start service"),
        Some(std_err),
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api/v1",
        routes![index, stop_service, start_service, service_status],
    )
}

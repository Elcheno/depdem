use crate::models::jwt_model::JWT;
use crate::models::response_model::{
    NetworkResponse, ResponseBody, ResponseError, ResponseSuccess,
};
use crate::utils::transform::transform_vec_to_string;
use rocket::serde::json::{json, Json};
use std::process::Command;

#[get("/service/status")]
pub fn service_status(
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
pub fn stop_service(
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
pub fn start_service(
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

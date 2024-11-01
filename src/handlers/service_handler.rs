use crate::models::jwt_model::JWT;
use crate::models::response_model::{
    NetworkResponse, ResponseBody, ResponseError, ResponseSuccess,
};
use crate::models::service_dto_model::ServiceRequest;
use crate::models::service_model::{ActionService, Service};
use crate::utils::transform::transform_vec_to_string;
use rocket::serde::json::{json, Json};

#[get("/service/status", format = "json", data = "<service_request>")]
pub fn service_status(
    key: Result<JWT, NetworkResponse>,
    service_request: Json<ServiceRequest>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;
    let service = Service::build(&service_request.into_inner(), ActionService::Status)?;
    let output = service.execute();
    let code = output.status.code().unwrap();
    if code == 0 {
        return Ok(Json(ResponseSuccess::ok("Service is running".to_string())));
    }
    Err(NetworkResponse::InternalServerError(
        json!(ResponseBody::Error(ResponseError::new(
            Some(code),
            String::from("Error checking service status"),
            Some(transform_vec_to_string(&output.stderr)),
        )))
        .to_string(),
    ))
}

#[put("/service/stop", format = "json", data = "<service_request>")]
pub fn stop_service(
    key: Result<JWT, NetworkResponse>,
    service_request: Json<ServiceRequest>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;
    let service = Service::build(&service_request.into_inner(), ActionService::Stop)?;
    let output = service.execute();
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

#[put("/service/start", format = "json", data = "<service_request>")]
pub fn start_service(
    key: Result<JWT, NetworkResponse>,
    service_request: Json<ServiceRequest>,
) -> Result<Json<ResponseSuccess>, NetworkResponse> {
    let _jwt = key?;
    let service = Service::build(&service_request.into_inner(), ActionService::Start)?;
    let output = service.execute();
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

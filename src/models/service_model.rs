use crate::models::response_model::{NetworkResponse, ResponseBody};
use crate::models::service_dto_model::ServiceRequest;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};
use std::process::{Command, Output};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Service {
    pub service_type: ServiceType,
    pub action: ActionService,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ServiceType {
    Tomcat10,
    Apache2,
}

impl ServiceType {
    pub fn name_service(&self) -> &'static str {
        match self {
            ServiceType::Tomcat10 => "tomcat10",
            ServiceType::Apache2 => "apache2",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ActionService {
    Start,
    Stop,
    Status,
    Restart,
}

impl ActionService {
    pub fn execute(&self, service_type: &ServiceType) -> Output {
        let name_service = service_type.name_service();
        let command = match self {
            ActionService::Start => format!("service {} start", name_service),
            ActionService::Stop => format!("service {} stop", name_service),
            ActionService::Status => format!("service {} status", name_service),
            ActionService::Restart => format!("service {} restart", name_service),
        };
        Command::new("sh")
            .args(["-c", &command])
            .output()
            .expect("failed to execute process")
    }
}

impl Service {
    pub fn new(service_type: ServiceType, action: ActionService) -> Self {
        Self {
            service_type,
            action,
        }
    }

    pub fn build(request: &ServiceRequest, action: ActionService) -> Result<Self, NetworkResponse> {
        match request.service_type.as_str() {
            "Tomcat10" => Ok(Service::new(ServiceType::Tomcat10, action)),
            "Apache2" => Ok(Service::new(ServiceType::Apache2, action)),
            _ => {
                return Err(NetworkResponse::BadRequest(
                    json!(ResponseBody::Message("Bad request".to_string())).to_string(),
                ))
            }
        }
    }

    pub fn execute(&self) -> Output {
        self.action.execute(&self.service_type)
    }
}

#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod models;
pub mod utils;

use dotenvy::dotenv;

use handlers::*;

use utils::keys::{generate_keys, load_keys};

#[get("/")]
fn index() -> &'static str {
    "depdem running"
}

fn init() -> () {
    match load_keys() {
        Ok(res) => println!("{:?}", res),
        Err(err) => {
            println!("{:?}", err);
            match generate_keys() {
                Ok(res) => println!("{:?}", res),
                Err(err) => println!("{:?}", err),
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    init();

    rocket::build().mount(
        "/depdem/api/v1",
        routes![
            index,
            service_handler::stop_service,
            service_handler::start_service,
            service_handler::service_status,
            auth_handler::login
        ],
    )
}

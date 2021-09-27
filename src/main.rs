#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};

#[get("/")]
fn index() -> String { "Works".to_string() }

#[launch]
fn build() -> Rocket<Build> {
    log4rs::init_file("log4rs.yml", Default::default()).expect("log init error");
    rocket::build().mount("/", routes![index])
}

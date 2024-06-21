#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

pub(crate) mod schema;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", routes::prelude::all_routes())
}

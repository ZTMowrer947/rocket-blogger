#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub(crate) mod schema;
pub(crate) mod models;
pub(crate) mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from("../public"))
        .mount("/", routes::prelude::all_routes())
}

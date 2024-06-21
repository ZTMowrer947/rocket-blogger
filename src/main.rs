#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};

pub(crate) mod schema;
pub(crate) mod models;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from("../public"))
        .mount("/", routes![index])
}

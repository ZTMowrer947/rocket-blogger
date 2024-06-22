use rocket::http::Status;
use rocket_dyn_templates::Template;

use crate::csp::NonceContext;

#[get("/new-post")]
pub fn new_post_form(nonces: &NonceContext) -> Template {
    Template::render("new", nonces)
}

#[post("/new-post")]
pub fn new_post() -> Status {
    Status::NotImplemented
}

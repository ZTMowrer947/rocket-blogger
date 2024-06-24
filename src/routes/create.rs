use diesel::Insertable;
use rocket::http::Status;
use rocket_dyn_templates::Template;

use crate::{csp::NonceContext, schema::posts};

// Form data for creating a new post
#[derive(FromForm, Insertable)]
#[diesel(table_name = posts)]
pub struct PostInput<'r> {
    #[field(validate = neq("").or_else(msg!("Title must not be empty")))]
    title: &'r str,
    #[field(validate = neq("").or_else(msg!("Body must not be empty")))]
    body: &'r str,
}

#[get("/new-post")]
pub fn new_post_form(nonces: &NonceContext) -> Template {
    Template::render("new", nonces)
}

#[post("/new-post")]
pub fn new_post() -> Status {
    Status::NotImplemented
}

use diesel::Insertable;
use rocket::form::{Contextual, Form, FromForm};
use rocket::response::Redirect;
use rocket_db_pools::{Connection, diesel::insert_into};
use rocket_dyn_templates::Template;

use crate::db::Blogger;
use crate::{csp::NonceContext, schema::posts};
use crate::models::post::Post;

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

#[post("/new-post", data = "<input>")]
pub async fn new_post(
    mut db: Connection<Blogger>,
    nonces: &NonceContext,
    input: Form<Contextual<PostInput<'_>, '_>>) -> Result<Redirect, Template> {
    // Parse form
    if let Some(ref post_data) = input.value {
        use rocket_db_pools::diesel::prelude::*;

        // If successful, create post from input data
        insert_into(posts::table)
            .values(post_data)
            .returning(Post::as_returning())
            .get_result(&mut db)
            .await
            .expect("Unexpected error during post creation");

        Ok(Redirect::to(uri!(super::root::index)))
    } else {
        println!("Validation context: {:?}", input.context);

        // TODO: Pass validation errors back to form
        Err(Template::render("new", nonces))
    }
}

use rocket_db_pools::{Connection,diesel::prelude::*};
use rocket_dyn_templates::{context, Template};

use crate::{db::Blogger, models::post::Post};

#[get("/")]
pub async fn index(mut db: Connection<Blogger>) -> Template {
    use crate::schema::posts::dsl::*;

    // Fetch list of posts
    let post_list = posts
        .select(Post::as_select())
        .load(&mut db)
        .await
        .expect("Could not retrieve posts from ");

    // Render page with nonces in context
    Template::render("index", context! {
        posts: post_list
    })
}

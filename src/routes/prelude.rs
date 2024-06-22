use rocket::Route;

use super::*;

pub fn all_routes() -> Vec<Route> {
    routes![root::index, create::new_post_form, create::new_post]
}

use rocket_dyn_templates::Template;

use crate::csp::NonceContext;

#[get("/")]
pub fn index(nonces: &NonceContext) -> Template {
    // Render page with nonces in context
    Template::render("index", nonces)
}

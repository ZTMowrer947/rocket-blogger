use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        style_nonces: vec!["style_placeholder"],
        script_nonces: vec!["script_placeholder"]
    })
}

use std::fs::read_to_string;
use std::path::Path;
use rocket::fs::relative;
use rocket_dyn_templates::Template;

use crate::csp::{AssetCounts, NonceContext};

#[get("/")]
pub fn index() -> Template {
    // Read count of number of scripts and stylesheets
    let path = Path::new(relative!("templates/generated/asset_counts.json"));
    let asset_count_str = read_to_string(&path).unwrap();
    let asset_counts: AssetCounts = serde_json::from_str(&asset_count_str).unwrap();

    // Generate nonces for scripts and stylesheets
    let nonce_context: NonceContext = asset_counts.into();

    // Render page with nonces in context
    Template::render("index", nonce_context)
}

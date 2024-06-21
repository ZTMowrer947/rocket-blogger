use std::fs::read_to_string;
use std::path::Path;
use base64::prelude::*;
use rocket::{fs::relative, serde::Deserialize};
use rocket_dyn_templates::{context, Template};
use rand::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AssetManifest {
    script_count: i32,
    style_count: i32,
}

#[get("/")]
pub fn index() -> Template {
    // Read count of number of scripts and stylesheets
    let path = Path::new(relative!("templates/generated/asset_counts.json"));
    let asset_count_str = read_to_string(&path).unwrap();
    let asset_counts: AssetManifest = serde_json::from_str(&asset_count_str).unwrap();

    // Closure for nonce generation
    let generate_nonce = |_: i32| -> String {
        let mut rng = thread_rng();
        let mut nonce_bytes = [0u8; 64];
        rng.fill_bytes(&mut nonce_bytes);

        BASE64_STANDARD.encode(nonce_bytes)
    };

    // Generate nonces for each stylesheet and script
    let style_nonces = (0..asset_counts.style_count)
        .map(generate_nonce)
        .collect::<Vec<_>>();

    let script_nonces = (0..asset_counts.script_count)
        .map(generate_nonce)
        .collect::<Vec<_>>();

    // Render page with nonces in context
    Template::render("index", context! {
        style_nonces,
        script_nonces
    })
}

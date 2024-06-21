use std::fs::read_to_string;
use std::path::Path;

use base64::prelude::*;
use rand::prelude::*;
use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::relative, http::{ContentType, Status},
    request::{self, FromRequest},
    serde::{Deserialize, Serialize},
    Request,
    Response
};

// Structs for asset nonces
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AssetCounts {
    script_count: i32,
    style_count: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NonceContext {
    pub script_nonces: Vec<String>,
    pub style_nonces: Vec<String>,
}

// Nonce generation helpers
fn generate_nonce() -> String {
    let mut rng = thread_rng();
    let mut nonce_bytes = [0u8; 64];
    rng.fill_bytes(&mut nonce_bytes);

    BASE64_URL_SAFE.encode(nonce_bytes)
}

impl From<AssetCounts> for NonceContext {
    fn from(asset_counts: AssetCounts) -> Self {
        let AssetCounts { script_count, style_count } = asset_counts;

        Self {
            script_nonces: (0..script_count)
                .map(|_| generate_nonce())
                .collect(),
            style_nonces: (0..style_count)
                .map(|_| generate_nonce())
                .collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r NonceContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let path = Path::new(relative!("templates/generated/asset_counts.json"));

        request::Outcome::Success(request.local_cache(|| {
            let asset_count_str = read_to_string(&path).unwrap();
            let asset_counts: AssetCounts = serde_json::from_str(&asset_count_str).unwrap();

            NonceContext::from(asset_counts)
        }))
    }
}

pub struct StrictCsp;

#[rocket::async_trait]
impl Fairing for StrictCsp {
    fn info(&self) -> Info {
        Info {
            name: "Strict Content Security Policy Manager",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Do not operate in debug mode
        if cfg!(debug_assertions) {
            return
        }
        
        // Do not operate on failed or non-HTML responses
        if response.status() != Status::Ok || response.content_type() != Some(ContentType::HTML) {
            return
        }

        let nonces = request.guard::<&NonceContext>().await.unwrap();

        // Construct CSP policy
        let mut policy = String::from("default-src 'none'; script-src ");

        // Add nonces for scripts if any
        if nonces.script_nonces.len() == 0 {
            policy.push_str("'none';")
        } else {
            for nonce in &nonces.script_nonces {
                policy.push_str(format!("'nonce-{nonce}' ").as_str())
            }

            policy.push(';');
        }

        // Add nonces for styles if any
        policy.push_str("style-src ");

        if nonces.style_nonces.len() == 0 {
            policy.push_str("'none'")
        } else {
            for nonce in &nonces.style_nonces {
                policy.push_str(format!("'nonce-{nonce}' ").as_str())
            }

            policy.push_str("'strict-dynamic' https:;");
        }

        // Attach CSP header to response
        response.set_raw_header("Content-Security-Policy", policy);
    }
}

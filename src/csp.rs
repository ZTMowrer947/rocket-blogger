use std::fs::read_to_string;
use std::path::Path;

use base64::prelude::*;
use rand::prelude::*;
use rocket::{fs::relative, request::{self, FromRequest}, serde::{Deserialize, Serialize}, Request};

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

fn generate_nonce() -> String {
    let mut rng = thread_rng();
    let mut nonce_bytes = [0u8; 64];
    rng.fill_bytes(&mut nonce_bytes);

    BASE64_STANDARD.encode(nonce_bytes)
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

use base64::prelude::*;
use rand::prelude::*;
use rocket::serde::{Deserialize, Serialize};


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

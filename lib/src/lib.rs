use uint::construct_uint;
use serde::{Deserialize, Serialize};

construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

pub mod sha256;
pub mod types;
pub mod util;
pub mod crypto;

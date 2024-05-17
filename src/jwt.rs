use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OhMyClaims {
    pub user_id: usize,
    pub username: String,
    pub exp: usize,
}

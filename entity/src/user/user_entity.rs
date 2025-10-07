use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: i64,
    pub name: String,
}

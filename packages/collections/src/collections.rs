use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// User struct
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Metadata struct

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub public_metadata: Value,
    pub private_metadata: Value,
    pub unsafe_metadata: Value,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            public_metadata: Value::Null,
            private_metadata: Value::Null,
            unsafe_metadata: Value::Null,
        }
    }
    pub fn set_public_metadata(&mut self, metadata: Value) {
        self.public_metadata = metadata;
    }
    pub fn set_private_metadata(&mut self, metadata: Value) {
        self.private_metadata = metadata;
    }
    pub fn set_unsafe_metadata(&mut self, metadata: Value) {
        self.unsafe_metadata = metadata;
    }
    pub fn check_size(&self) -> bool {
        // True if size is within limit
        let public_size = self.public_metadata.to_string().len();
        let private_size = self.private_metadata.to_string().len();
        let unsafe_size = self.unsafe_metadata.to_string().len();
        let total_size = public_size + private_size + unsafe_size;
        return total_size < 8 * 1024; // 8 KB limit. This can be adjusted as needed but it can cause migration issues because of the size limit.
    }
}

// Session tokens struct

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub token_id: Uuid,
    pub user_id: String,
    pub created_at: String,
    pub expires_at: String,
}

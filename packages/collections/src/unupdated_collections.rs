use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::SystemTime;
use svix_ksuid::Ksuid;
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
    pub atz: Option<String>, // The authorized party. Example: "https://example.com"
    pub exp: SystemTime,     // The time when the session token expires
    pub fva: Vec<isize>, // The fva means for the factor verification age. That means the age of the factor verification. Example: [3, 2] -> In this case, the first factor was verified 3 minutes ago and the second factor was verified 2 minutes ago. First factor is the password, second factor is the MFA. This is used to determine if the user needs to re-verify their factors before commiting a change.
    pub iat: SystemTime, // The time when the session token was issued
    pub iss: String,     // The issuer of the session token. Example: "https://example.com"
    pub jti: Ksuid, // The unique identifier for the session token. We use Ksuid because it is a globally unique identifier that is sortable by time.
    pub nbf: SystemTime, // The time before which the session token is not valid. This is used to prevent replay attacks.
    pub sid: String, // The session ID. This is used to identify the session. Example: "sess_1a1f5g5h669988". The session ID is a nano ID that is unique to the session. It is used to identify the session in the database. It is prefixed by default with "sess_" to indicate that it is a session ID.
    pub sub: String, // The subject of the session token. This is the user ID of the user that the session token belongs to. Example: "user_1a1f5g5h669988". The subject is a nano ID that is unique to the user. It is used to identify the user in the database. It is prefixed by default with "user_" to indicate that it is a user ID.
    pub v: u8, // The version of the session token. This is used to determine the version of the session token. Default is 1.
    pub pla: Vec<String>, // The user's plan. This is used to determine the user's plan. It is a vector of URNs that represent the user's plan. Example: ["o:free", "u:pro"]. This can also be prefixed with "uo:" for user and organization features.
    pub fea: Vec<String>, // The user's features. This is used to determine the user's features. It is a vector of URNs that represent the user's features. Example: ["o:feature1", "u:feature2"]. This can also be prefixed with "uo:" for user and organization features.
    pub o: Option<Organization>,
    pub act: Option<Actor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Features {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Plans {
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    // The actor is only used while impersonating a user. It is used to identify the user that is being impersonated and the one that's impersonating. This is used to determine the user's permissions and roles in the organization.
    pub iss: String, // The issuer of the actor. This is the user ID of the user that the actor belongs to. Example: "user_1a1f5g5h669988". The issuer is a nano ID that is unique to the user. It is used to identify the user in the database. It is prefixed by default with "user_" to indicate that it is a user ID.
    pub sid: String, // The session ID of the actor. This is used to identify the session that the actor belongs to. Example: "sess_1a1f5g5h669988". The session ID is a nano ID that is unique to the session. It is used to identify the session in the database. It is prefixed by default with "sess_" to indicate that it is a session ID.
    pub sub: String, // The subject of the actor. This is the user ID of the user that the actor belongs to. Example: "user_1a1f5g5h669988". The subject is a nano ID that is unique to the user. It is used to identify the user in the database. It is prefixed by default with "user_" to indicate that it is a user ID.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    id: String, // The organization ID. This is the ID of the organization that the user belongs to. Example: "org_1a1f5g5h669988". The organization ID is a nano ID that is unique to the organization. It is used to identify the organization in the database. It is prefixed by default with "org_" to indicate that it is an organization ID.
    slg: String, // The organization slug. This is a unique identifier for the organization that is used in URLs. Example: "my-organization". It is a human-readable identifier that is used to identify the organization in the database.
    rol: Vec<Role>, // The roles of the user in the organization. This is a vector of roles that the user has in the organization. Example: ["admin", "member"]. This is used to determine the user's roles in the organization. It is a vector of strings that represent the user's roles in the organization.
    pub perm: Vec<Permission>, // The permissions of the user in the organization. This is a vector of permissions that the user has in the organization. Example: ["read", "write"]. This is used to determine the user's permissions in the organization. It is a vector of strings that represent the user's permissions in the organization.
    pub fpm: Vec<u8>, // The feature permissions of the user in the organization. This takes the vector of permissions and uses this to know if you have access to a feature or not. Example: [1, 0, 1] -> The user has access to the first and third features, but not the second one. This is used to determine the user's feature permissions in the organization. It is a vector of u8 that represent the user's feature permissions in the organization.
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Member,
    Guest,
}

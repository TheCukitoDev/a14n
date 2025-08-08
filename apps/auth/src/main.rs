use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::query;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
struct DefaultResponse {
    request_id: Option<String>, // The "request_id" member is a JSON string containing a unique identifier for the request that caused the error. This can be used for debugging and tracing purposes. The method of generating this ID is implementation-specific.
    title: String, // A short, human-readable summary of the response. Or in case of an error, a short description of the error.
    status: Option<u16>, // The same status code as the HTTP response. This is an integer that indicates the HTTP status code of the response.
}

// The error response structure that follows the RFC 9457 specification. See: https://datatracker.ietf.org/doc/html/rfc9457
#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    #[serde(flatten)]
    default: DefaultResponse, // The default response structure that contains response common fields.
    r#type: String, // The URI reference to the error type. This should be a valid URI to the error type definition on the documentation site.
    detail: Option<String>, // A human-readable explanation specific to this occurrence of the problem. This is a string that provides more context about the error.
    instance: Option<String>, // A URI reference that identifies the specific occurrence of the problem. This can be used to provide more information about the error instance. Example: "/api/v1/auth"
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidationError {
    /// The error response details following RFC 9457.
    #[serde(flatten)]
    error: ErrorResponse,
    /// A list of validation errors, each with a pointer and detail as per RFC 9457.
    errors: Vec<ValidationErrorDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidationErrorDetail {
    detail: String,  // A human-readable explanation of the validation error.
    pointer: String, // A JSON Pointer that identifies the location of the error in the request body.
}

#[derive(Serialize, Deserialize, Debug)]
struct SuccessResponse {
    #[serde(flatten)]
    default: DefaultResponse, // The default response structure that contains response common fields.
    username: String, // The username of the authenticated user.
    user_id: String,  // The unique identifier of the authenticated user.
    timestamp: u64,   // The date and time of the authentication.
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthorizeRequestPayload {
    username: String,
    password: String,
}

async fn authenticate_user(Json(payload): Json<AuthorizeRequestPayload>) -> Response {
    println!(
        "Received authentication request for user: {}",
        payload.username
    );

    if payload.password == "password123" {
        let success = SuccessResponse {
            default: DefaultResponse {
                request_id: Some("12345".to_string()),
                title: "Authentication Successful".to_string(),
                status: Some(200),
            },
            username: payload.username,
            user_id: "user-123".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        (StatusCode::OK, axum::Json(success)).into_response()
    } else {
        let error = ValidationError {
            error: ErrorResponse {
                default: DefaultResponse {
                    request_id: Some("12345".to_string()),
                    title: "Authentication Failed".to_string(),
                    status: Some(401),
                },
                r#type: "https://example.com/errors/authentication-failed".to_string(),
                detail: Some("Invalid username or password.".to_string()),
                instance: Some("/api/v1/auth".to_string()),
            },
            errors: vec![ValidationErrorDetail {
                detail: "Invalid password.".to_string(),
                pointer: "#/password".to_string(),
            }],
        };
        (StatusCode::UNAUTHORIZED, axum::Json(error)).into_response()
    }
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://koyeb-adm:npg_ZXMY8bGqfAE2@ep-twilight-band-a2mk9tpa.eu-central-1.pg.koyeb.app/koyebdb") // Replace with your database connection string
        .await
        .unwrap();

    let query = query!("SELECT * FROM users WHERE username = $1", "testuser")
        .fetch_one(&pool)
        .await;

    println!("Query result: {:?}", query);

    let app = Router::new()
        .route("/", get(|| async { "Welcome to the Auth Service!" }))
        .route("/v1/auth", post(authenticate_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 a14n Auth service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

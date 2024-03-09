use axum::response::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct Task {
    id: u32,
    name: String,
    description: String,
    deadline: String,
    completed: bool,
}

async fn index() -> Result<impl IntoResponse, MyError> {
    let tasks: Vec<Task> = vec![];
    Ok(Json(tasks))
}

#[derive(Error, Debug)]
enum MyError {}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

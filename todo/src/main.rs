use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    name: String,
    description: String,
    deadline: String,
    completed: bool,
}

async fn index(State(tasks): State<Arc<Mutex<Vec<Task>>>>) -> Result<impl IntoResponse, MyError> {
    Ok(Json(tasks.lock().unwrap().to_vec()))
}

#[derive(Deserialize)]
struct AddRequest {
    name: String,
    description: String,
    deadline: String,
}

#[derive(Serialize)]
struct AddResponse {
    id: u32,
}

async fn add_post(
    State(tasks): State<Arc<Mutex<Vec<Task>>>>,
    Json(req): Json<AddRequest>,
) -> Result<impl IntoResponse, MyError> {
    let id = tasks.lock().unwrap().len() as u32;
    tasks.lock().unwrap().push(Task {
        id,
        name: req.name,
        description: req.description,
        deadline: req.deadline,
        completed: false,
    });
    Ok(Json(AddResponse { id }))
}

#[derive(Deserialize)]
struct DeleteRequest {
    id: u32,
}

async fn delete_todo(
    State(tasks): State<Arc<Mutex<Vec<Task>>>>,
    Json(req): Json<DeleteRequest>,
) -> Result<impl IntoResponse, MyError> {
    let id = req.id as usize;
    if id >= tasks.lock().unwrap().len() {
        return Ok(StatusCode::BAD_REQUEST);
    }
    tasks.lock().unwrap().remove(id);
    Ok(StatusCode::NO_CONTENT)
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
    let tasks: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/", get(index).post(add_post).delete(delete_todo))
        .with_state(tasks);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

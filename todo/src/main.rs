use axum::{
    async_trait,
    extract::Host,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use openapi::{
    models::{self, PostUsers201Response},
    server::new,
    Api, DeleteTasksResponse, GetTasksResponse, PostAuthResponse, PostTasksResponse,
    PostUsersResponse, PutTasksResponse,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Serialize, Clone)]
struct User {
    id: u32,
    email: String,
    password: String,
}

#[derive(Clone)]
struct ApiImpl {
    users: Arc<Mutex<Vec<User>>>,
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}

#[async_trait]
impl Api for ApiImpl {
    async fn post_users(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::UserCredentials>,
    ) -> Result<PostUsersResponse, String> {
        match body {
            None => Err("body is required".to_string()),
            Some(body) => {
                let id = self.users.lock().unwrap().len() as u32;
                let user = User {
                    id,
                    email: body.email,
                    password: body.password,
                };
                self.users.lock().unwrap().push(user);
                Ok(PostUsersResponse::Status201(PostUsers201Response {
                    id: Some(id as i64),
                }))
            }
        }
    }
    async fn post_auth(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Option<models::UserCredentials>,
    ) -> Result<PostAuthResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn post_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Option<models::PostTasksRequest>,
    ) -> Result<PostTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn get_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: models::GetTasksQueryParams,
    ) -> Result<GetTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn delete_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::DeleteTasksPathParams,
    ) -> Result<DeleteTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn put_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::PutTasksPathParams,
        _body: Option<models::PutTasksRequest>,
    ) -> Result<PutTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
}

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    name: String,
    description: String,
    deadline: String,
    completed: bool,
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
    let router = new(ApiImpl {
        users: Arc::new(Mutex::new(vec![])),
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}

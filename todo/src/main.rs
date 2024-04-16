use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    models::{
        self, DeleteTasksHeaderParams, GetTasksHeaderParams, PostTasksHeaderParams,
        PostUsers201Response, PutTasksHeaderParams, Token,
    },
    server::new,
    Api, DeleteTasksResponse, GetTasksResponse, PostAuthResponse, PostTasksResponse,
    PostUsersResponse, PutTasksResponse,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use validator::Validate;
mod jwt;

const SECRET: &str = "secret";

#[derive(Deserialize, Serialize, Clone, Validate)]
struct User {
    id: u32,
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Clone)]
struct ApiImpl {
    users: Arc<Mutex<Vec<User>>>,
    tasks: Arc<Mutex<HashMap<u32, Vec<Task>>>>,
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
        let (email, password) = match body {
            None => Err("body is required".to_string()),
            Some(body) => Ok((body.email, body.password)),
        }?;
        if self
            .users
            .lock()
            .unwrap()
            .iter()
            .any(|user| user.email == email)
        {
            return Ok(PostUsersResponse::Status400_BadRequest);
        }
        let id = self.users.lock().unwrap().len() as u32;
        let user = User {
            id,
            email,
            password,
        };
        if user.validate().is_err() {
            return Ok(PostUsersResponse::Status400_BadRequest);
        }
        self.users.lock().unwrap().push(user);

        Ok(PostUsersResponse::Status201(PostUsers201Response {
            id: Some(id as i64),
        }))
    }
    async fn post_auth(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::UserCredentials>,
    ) -> Result<PostAuthResponse, String> {
        let (email, password) = match body {
            None => Err("body is required".to_string()),
            Some(body) => Ok((body.email, body.password)),
        }?;
        let users_locked = self.users.lock().unwrap();

        let user = users_locked
            .iter()
            .find(|user| user.email == email && user.password == password);
        if let Some(user) = user {
            let token = jwt::jwt::create_token(&SECRET.as_ref(), &user.id.to_string())
                .map_err(|e| e.to_string())?;
            Ok(PostAuthResponse::Status200(Token { token: Some(token) }))
        } else {
            Ok(PostAuthResponse::Status400_BadRequest)
        }
    }
    async fn post_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        headers: PostTasksHeaderParams,
        body: Option<models::PostTasksRequest>,
    ) -> Result<PostTasksResponse, String> {
        let jwt = headers.authorization.replace("Bearer ", "");
        let claims = match jwt::jwt::validate_token(&SECRET.as_ref(), &jwt) {
            Ok(claims) => claims,
            Err(_) => return Ok(PostTasksResponse::Status401_Unauthorized),
        };
        let user_id = match claims.uid.parse::<u32>() {
            Ok(user_id) => user_id,
            Err(_) => return Ok(PostTasksResponse::Status401_Unauthorized),
        };

        let users_locked = self.users.lock().unwrap();
        if users_locked
            .iter()
            .find(|user| user.id == user_id)
            .is_none()
        {
            return Ok(PostTasksResponse::Status400_BadRequest);
        }

        let (name, description, deadline, completed) = match body {
            None => Err("body is required".to_string()),
            Some(body) => Ok((
                body.name,
                body.description,
                body.deadline,
                body.completed.unwrap_or(false),
            )),
        }?;
        let mut tasks_unlocked = self.tasks.lock().unwrap();
        let mut user_tasks = tasks_unlocked.get(&user_id).unwrap_or(&vec![]).clone();
        let task_id = user_tasks.len() as u32;
        let task = Task {
            id: task_id,
            name,
            description,
            deadline: deadline.to_string(),
            completed,
        };
        user_tasks.push(task);
        tasks_unlocked.insert(user_id, user_tasks.clone());
        Ok(PostTasksResponse::Status201(models::TaskId {
            id: Some(task_id as i64),
        }))
    }
    async fn get_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _headers: GetTasksHeaderParams,
        _query_params: models::GetTasksQueryParams,
    ) -> Result<GetTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn delete_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _headers: DeleteTasksHeaderParams,
        _path_params: models::DeleteTasksPathParams,
    ) -> Result<DeleteTasksResponse, String> {
        Err("not implemented yet".to_string())
    }
    async fn put_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _headers: PutTasksHeaderParams,
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

#[tokio::main]
async fn main() {
    let router = new(ApiImpl {
        users: Arc::new(Mutex::new(vec![])),
        tasks: Arc::new(Mutex::new(HashMap::new())),
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}

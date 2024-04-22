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
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use validator::Validate;
mod domains;
mod jwt;

const SECRET: &str = "secret";

#[derive(Clone)]
struct ApiImpl {
    users: Arc<Mutex<Vec<domains::User>>>,
    tasks: Arc<Mutex<HashMap<u32, Vec<domains::Task>>>>,
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
        let user = domains::User {
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
        let task = domains::Task {
            id: task_id,
            name,
            description,
            deadline: deadline.to_rfc3339(),
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
        headers: GetTasksHeaderParams,
        query_params: models::GetTasksQueryParams,
    ) -> Result<GetTasksResponse, String> {
        let jwt = headers.authorization.replace("Bearer ", "");
        match jwt::jwt::validate_token(&SECRET.as_ref(), &jwt) {
            Ok(_) => {}
            Err(_) => return Ok(GetTasksResponse::Status401_Unauthorized),
        };
        let user_id = query_params.user_id as u32;

        let users_locked = self.users.lock().unwrap();
        if users_locked
            .iter()
            .find(|user| user.id == user_id)
            .is_none()
        {
            return Ok(GetTasksResponse::Status400_BadRequest);
        }

        let tasks_unlocked = self
            .tasks
            .lock()
            .unwrap()
            .get(&user_id)
            .unwrap_or(&vec![])
            .clone();
        let tasks: Vec<models::Task> = tasks_unlocked
            .iter()
            .map(|task| models::Task {
                id: Some(task.id as i64),
                name: Some(task.name.clone()),
                description: Some(task.description.clone()),
                deadline: match chrono::DateTime::parse_from_rfc3339(&task.deadline) {
                    Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
                    Err(_) => None,
                },
                completed: Some(task.completed),
            })
            .collect();

        Ok(GetTasksResponse::Status200(tasks.clone()))
    }
    async fn delete_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        headers: DeleteTasksHeaderParams,
        path_params: models::DeleteTasksPathParams,
    ) -> Result<DeleteTasksResponse, String> {
        let jwt = headers.authorization.replace("Bearer ", "");
        let claims = match jwt::jwt::validate_token(&SECRET.as_ref(), &jwt) {
            Ok(claims) => claims,
            Err(_) => return Ok(DeleteTasksResponse::Status401_Unauthorized),
        };
        let user_id = match claims.uid.parse::<u32>() {
            Ok(user_id) => user_id,
            Err(_) => return Ok(DeleteTasksResponse::Status401_Unauthorized),
        };

        let users_locked = self.users.lock().unwrap();
        if users_locked
            .iter()
            .find(|user| user.id == user_id)
            .is_none()
        {
            return Ok(DeleteTasksResponse::Status400_BadRequest);
        }

        let task_id = path_params.task_id;
        let mut tasks_unlocked = self.tasks.lock().unwrap();
        if tasks_unlocked.get(&user_id).is_none() {
            return Ok(DeleteTasksResponse::Status404_NotFound);
        }
        tasks_unlocked
            .get_mut(&user_id)
            .map(|tasks| {
                tasks.retain(|task| task.id != task_id as u32);
            })
            .unwrap_or(());
        Ok(DeleteTasksResponse::Status204_DeleteSucceeded)
    }
    async fn put_tasks(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        headers: PutTasksHeaderParams,
        path_params: models::PutTasksPathParams,
        body: Option<models::PutTasksRequest>,
    ) -> Result<PutTasksResponse, String> {
        let jwt = headers.authorization.replace("Bearer ", "");
        let claims = match jwt::jwt::validate_token(&SECRET.as_ref(), &jwt) {
            Ok(claims) => claims,
            Err(_) => return Ok(PutTasksResponse::Status401_Unauthorized),
        };
        let user_id = match claims.uid.parse::<u32>() {
            Ok(user_id) => user_id,
            Err(_) => return Ok(PutTasksResponse::Status401_Unauthorized),
        };

        let users_locked = self.users.lock().unwrap();
        if users_locked
            .iter()
            .find(|user| user.id == user_id)
            .is_none()
        {
            return Ok(PutTasksResponse::Status400_BadRequest);
        }

        let task_id = path_params.task_id as u32;
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
        if tasks_unlocked.get(&user_id).is_none() {
            return Ok(PutTasksResponse::Status404_NotFound);
        }
        let task_to_update = tasks_unlocked
            .get(&user_id)
            .unwrap()
            .iter()
            .find(|task| task.id == task_id)
            .unwrap();
        let task_updated = domains::Task {
            id: task_id,
            name: name
                .clone()
                .get_or_insert(task_to_update.name.clone())
                .clone(),
            description: description
                .clone()
                .get_or_insert(task_to_update.description.clone())
                .clone(),
            deadline: if deadline.is_none() {
                task_to_update.deadline.clone()
            } else {
                deadline.unwrap().to_rfc3339()
            },
            completed,
        };
        tasks_unlocked
            .get_mut(&user_id)
            .map(|tasks| {
                tasks.retain(|task| task.id != task_id);
                tasks.push(task_updated.clone());
            })
            .unwrap_or(());
        tasks_unlocked
            .get_mut(&user_id)
            .unwrap()
            .sort_by(|a, b| a.id.cmp(&b.id));
        Ok(PutTasksResponse::Status200(models::Task {
            id: Some(task_id as i64),
            name: Some(task_updated.clone().name),
            description: Some(task_updated.clone().description),
            deadline: match chrono::DateTime::parse_from_rfc3339(
                task_updated.clone().deadline.as_str(),
            ) {
                Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
                Err(_) => None,
            },
            completed: Some(task_updated.clone().completed),
        }))
    }
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

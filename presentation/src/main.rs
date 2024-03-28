use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use infra::in_memory::{InMemoryRoomRepository, InMemoryUserRepository};
use serde_json::Value;
use usecase::{
    room::{AddMemberError, AddMemberService, CreateRoomService, GetRoomService, RoomDto},
    user::{CreateUserService, GetUserError, GetUserService, UserDto},
};

#[tokio::main]
async fn main() {
    let user_repository = Arc::new(InMemoryUserRepository::new());
    let room_repository = Arc::new(InMemoryRoomRepository::new());
    let app = Router::new()
        .route("/user/:id", get(get_user))
        .route("/user/create/:name", post(create_user))
        .with_state(user_repository)
        .route("/room/:id", get(get_room))
        .route("/room/create/:name", post(create_room))
        .route("/room/join/:room_id", post(add_member))
        .with_state(room_repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Path(id): Path<String>,
) -> Result<Json<UserDto>, StatusCode> {
    let service = GetUserService::new(repository);
    match service.get_user(&id) {
        Ok(user) => Ok(Json(user)),
        Err(GetUserError::UserNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
async fn create_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Path(name): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<UserDto>, StatusCode> {
    let service = CreateUserService::new(repository);
    match service.create_user(&name) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_room(
    State(repository): State<Arc<InMemoryRoomRepository>>,
    Path(name): Path<String>,
) -> Result<Json<RoomDto>, StatusCode> {
    let service = CreateRoomService::new(repository);
    match service.create_room(&name) {
        Ok(room) => Ok(Json(room)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_room(
    State(repository): State<Arc<InMemoryRoomRepository>>,
    Path(id): Path<String>,
) -> Result<Json<RoomDto>, StatusCode> {
    let service = GetRoomService::new(repository);
    match service.get_room(&id) {
        Ok(room) => Ok(Json(room)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn add_member(
    State(repository): State<Arc<InMemoryRoomRepository>>,
    Path(room_id): Path<String>,
    Json(user): Json<UserDto>,
) -> Result<Json<RoomDto>, StatusCode> {
    let service = AddMemberService::new(repository);
    match service.add_member(&room_id, user) {
        Ok(room) => Ok(Json(room)),
        Err(AddMemberError::UserNotFound) => Err(StatusCode::NOT_FOUND),
        Err(AddMemberError::RoomNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

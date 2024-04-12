use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, query, query_as, PgPool, Pool, Postgres};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Stata {
    name: String,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("The 'DATABASE_URL'  value is required in the env");
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Could not connect to db");
    let app = Router::new()
        .route("/", get(highfive_world))
        .route("/users/:id", get(get_user))
        .route("/users", get(get_all_users).post(create_user))
        .with_state(AppState { pool });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn highfive_world() -> impl IntoResponse {
    "Nice one world"
}

async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let res = query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1;
    ",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .unwrap();

    (StatusCode::OK, Json(json!(res[0])))
}

async fn get_all_users(State(state): State<AppState>) -> impl IntoResponse {
    let res = query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!(res)))
}

async fn create_user(State(state): State<AppState>, Json(user): Json<User>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    query("INSERT INTO users VALUES($1, $2, $3, $4)")
        .bind(id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .execute(&state.pool)
        .await
        .unwrap();
    (
        StatusCode::CREATED,
        Json(json!({"message": "User created successfully"})),
    )
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: Option<Uuid>,
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]

struct Event {
    id: Uuid,
    price: f32,
    name: String,
    address: String,
    start_date: String,
    end_date: String,
    user: User,
    description: String,
    category: String,
}

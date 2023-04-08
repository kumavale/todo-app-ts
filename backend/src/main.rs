use std::sync::Arc;
use std::net::SocketAddr;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;

const DB_URL: &str = "sqlite://todos.db";

#[derive(sqlx::FromRow)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Todos {
    id: String,
    content: String,
    done: bool,
}

async fn get_all_todos_data(
    State(db): State<Arc<SqlitePool>>,
) -> String {
    let mut conn = db.acquire().await.unwrap();
    let todos = sqlx::query_as::<_, Todos>("SELECT * FROM todos;")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    serde_json::to_string(&todos).unwrap()
}

#[axum_macros::debug_handler]
async fn add_todo_data(
    State(pool): State<Arc<SqlitePool>>,
    Json(data): Json<Todos>,
) -> Json<Todos> {
    let response = data.clone();
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query(r#"INSERT INTO todos VALUES (?1, ?2, ?3);"#)
        .bind(data.id)
        .bind(data.content)
        .bind(data.done)
        .execute(&mut conn)
        .await
        .unwrap();
    Json(response)
}

async fn delete_todo_data(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query(r#"DELETE FROM todos WHERE id = ?1;"#)
        .bind(id)
        .execute(&mut conn)
        .await
        .unwrap();
    ""
}

async fn update_todo_data(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<String>,
    Json(data): Json<Todos>,
) -> Json<Todos> {
    let response = data.clone();
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query(r#"UPDATE todos SET done = ?1 WHERE id = ?2;"#)
        .bind(data.done)
        .bind(id)
        .execute(&mut conn)
        .await
        .unwrap();
    Json(response)
}

#[tokio::main]
async fn main() {
    // DBの作成
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }
    // テーブルの作成
    let pool = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS todos (id TEXT PRIMARY KEY, content TEXT, done INTEGER);"#)
        .execute(&pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/todos", get(get_all_todos_data))
        .route("/todos", post(add_todo_data))
        .route("/todos/:id", delete(delete_todo_data))
        .route("/todos/:id", put(update_todo_data))
        .layer(
            // FIXME: CORSの設定を見直す
            CorsLayer::permissive()
            //CorsLayer::new()
            //    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            //    .allow_methods([Method::GET, Method::POST])
        )
        .with_state(Arc::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3100));
    eprintln!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

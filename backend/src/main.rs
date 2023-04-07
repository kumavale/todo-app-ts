use std::sync::Arc;

use axum::{
    Extension,
    http::HeaderValue,
    routing::get,
    Router,
};
use http::{Method};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;

const DB_URL: &str = "sqlite://todos.db";

#[derive(sqlx::FromRow)]
#[derive(Debug, Serialize, Deserialize)]
struct Todos {
    id: String,
    content: String,
    done: bool,
}

async fn get_all_todos_data(
    Extension(db): Extension<Arc<SqlitePool>>,
) -> String {
    let mut conn = db.acquire().await.unwrap();
    let todos = sqlx::query_as::<_, Todos>("SELECT * FROM todos;")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    serde_json::to_string(&todos).unwrap()
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
        .layer(Extension(Arc::new(pool)))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
        );

    eprintln!("listening on localhost:3100");
    axum::Server::bind(&"127.0.0.1:3100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

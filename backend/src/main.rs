use axum::{
    http::HeaderValue,
    routing::get,
    Router,
};
use http::{Method};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}

async fn hello() -> String {
r#"[
  {
    "id": 1,
    "content": "Create react appをインストールする",
    "done": true
  },
  {
    "id": 2,
    "content": "JSON Server仮のAPIを作成する",
    "done": false
  },
  {
    "id": 3,
    "content": "Chakra UIをインストールする",
    "done": false
  }
]"#.into()
}

//async fn post_message(msg: serde_json::Json<Message>) -> String {
//    let message = msg.into_inner();
//    format!("Received message: {}", message.text)
//}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/todos", get(hello))
        //.route("/messages", post(post_message));
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
        )
        ;

    eprintln!("listening on localhost:3100");
    axum::Server::bind(&"127.0.0.1:3100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

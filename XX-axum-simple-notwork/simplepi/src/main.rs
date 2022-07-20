use std::panic::AssertUnwindSafe;

use articles::Articles;
use serde_json::{json, Value};

use axum::{
    Json,
    response::{Response},
    body::{Full, Bytes},
    http::StatusCode,
};

use axum_macros::debug_handler;

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.

    let article1: articles::Article = articles::Article { title: ("Cesta tam a zase zpatky".to_string()),
            author: ("J.R.R Tolkiej".to_string()), 
            description: ("Pohadkove vypraveni o hobitech".to_string()), 
            content: ("Fantasy".to_string()) };

    let mut articles = articles::Articles::new();

    articles.add(&article1);

    let app = axum::Router::new()
        .route("/",
            axum::routing::get(|| async { "Hello, World!" })
        )
        .route("/articles",
        axum::routing::get(get_article(&articles))
        );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn get_article(articles: &'art articles::Article) -> Response<Full<Bytes>> {
    // insert your application logic here
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Full::from(json!(articles.get(1))))
        .unwrap()
}
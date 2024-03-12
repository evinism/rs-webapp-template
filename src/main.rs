use axum::{
    routing::get,
    Router,
};

use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let static_service  = ServeDir::new("dist").not_found_service(ServeFile::new("assets/index.html"));
    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello, World!" }))
        .fallback_service(static_service)
    ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
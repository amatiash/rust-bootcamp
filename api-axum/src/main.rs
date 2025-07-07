use log::info;
use sqlx::postgres::PgPoolOptions;

use axum::{
    Router,
    routing::{delete, get, post},
};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let recs = sqlx::query("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch questions");

    info!("********* Question Records *********");
    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

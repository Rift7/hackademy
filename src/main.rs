mod db;
mod models;
mod routes;
mod utils;

use poem::{
    get, post, handler, http::Method, listener::TcpListener, EndpointExt, Route,
};
use poem::middleware::Tracing;
use askama::Template;
use anyhow::Result;
use sqlx::Pool;
use sqlx::Sqlite;

use crate::db::init_db;
use crate::utils::config::AppConfig;
use routes::{
    auth::*,
    category::get_categories,
    subcategory::get_subcategories,
    quiz::{get_quiz, submit_quiz},
    search::search_handler,
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    title: String,
}

#[handler]
async fn home_page() -> String {
    let tmpl = HomeTemplate {
        title: "Hackademy - Home".to_string(),
    };
    tmpl.render().unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env();
    let db_pool: Pool<Sqlite> = init_db(&config.database_url).await?;

    let session_store = SessionStore::new();

    let app = Route::new()
        // Home
        .at("/", get(home_page))
        // Categories
        .at("/categories", get(get_categories))
        .at("/category/:cat_id", get(get_subcategories))
        // Quiz
        .at("/quiz", get(get_quiz))
        .at("/quiz/submit", post(submit_quiz))
        // Search
        .at("/search", get(search_handler))
        // Auth
        .at("/auth/register", get(register_form).post(register_user))
        .at("/auth/login", get(login_form).post(login_user))
        .at("/auth/profile", get(profile))
        .at("/auth/logout", get(logout))

        .data(db_pool)
        .data(session_store)
        .with(Tracing);

    println!("Hackademy listening on {}", config.server_addr);
    poem::Server::new(TcpListener::bind(&config.server_addr))
        .run(app)
        .await?;

    Ok(())
}
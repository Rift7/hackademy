use poem::{
    handler,
    web::{Form, Data},
    http::{header::SET_COOKIE, StatusCode},
    IntoResponse, Request,
};
use askama::Template;
use sqlx::{Pool, Sqlite};
use nanoid::nanoid;
use std::sync::{Arc, Mutex};

use crate::models::User;
use crate::utils::security::{hash_password, verify_password};

#[derive(Debug, Clone)]
pub struct SessionStore {
    pub sessions: Arc<Mutex<std::collections::HashMap<String, String>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub fn set_session(&self, session_id: &str, user_id: &str) {
        let mut guard = self.sessions.lock().unwrap();
        guard.insert(session_id.to_string(), user_id.to_string());
    }

    pub fn get_user_id(&self, session_id: &str) -> Option<String> {
        let guard = self.sessions.lock().unwrap();
        guard.get(session_id).cloned()
    }

    pub fn remove_session(&self, session_id: &str) {
        let mut guard = self.sessions.lock().unwrap();
        guard.remove(session_id);
    }
}

#[derive(Template)]
#[template(path = "auth_register.html")]
struct RegisterTemplate {
    error: Option<String>,
}

#[derive(Template)]
#[template(path = "auth_login.html")]
struct LoginTemplate {
    error: Option<String>,
}

#[derive(Template)]
#[template(path = "auth_profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
}

#[derive(Debug, serde::Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[handler]
pub async fn register_form() -> impl IntoResponse {
    let tmpl = RegisterTemplate { error: None };
    tmpl.render().unwrap()
}

#[handler]
pub async fn register_user(db: Data<&Pool<Sqlite>>, Form(form): Form<RegisterForm>) -> impl IntoResponse {
    // Check if username is taken
    let existing: Option<User> = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&form.username)
    .fetch_optional(&**db)
    .await
    .unwrap();

    if existing.is_some() {
        let tmpl = RegisterTemplate { error: Some("Username already taken".into()) };
        return tmpl.render().unwrap();
    }

    // Hash password
    let hashed = match hash_password(&form.password) {
        Ok(h) => h,
        Err(_) => {
            let tmpl = RegisterTemplate { error: Some("Error hashing password".to_string()) };
            return tmpl.render().unwrap();
        }
    };

    // Create user record
    let user_id = nanoid!();
    sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&user_id)
    .bind(&form.username)
    .bind(&hashed)
    .execute(&**db)
    .await
    .unwrap();

    // Redirect to login
    poem::Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/auth/login")
        .body("User created, please log in.")
}

#[handler]
pub async fn login_form() -> impl IntoResponse {
    let tmpl = LoginTemplate { error: None };
    tmpl.render().unwrap()
}

#[handler]
pub async fn login_user(
    db: Data<&Pool<Sqlite>>,
    Form(form): Form<LoginForm>,
    session_store: Data<&SessionStore>,
) -> impl IntoResponse {
    let user: Option<User> = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&form.username)
    .fetch_optional(&**db)
    .await
    .unwrap();

    if let Some(u) = user {
        if verify_password(&form.password, &u.password_hash) {
            let session_id = nanoid!();
            session_store.set_session(&session_id, &u.id);

            let cookie_value = format!("hackademy_session_id={}; Path=/; HttpOnly", session_id);

            return (
                StatusCode::FOUND,
                [("Location", "/auth/profile"), (SET_COOKIE, cookie_value)],
                "Login successful!",
            );
        }
    }

    // Invalid user/pass
    let tmpl = LoginTemplate {
        error: Some("Invalid username or password".into()),
    };
    tmpl.render().unwrap()
}

#[handler]
pub async fn profile(req: &Request, db: Data<&Pool<Sqlite>>, session_store: Data<&SessionStore>) -> impl IntoResponse {
    let sid = get_session_id_from_cookie(req);
    if let Some(session_id) = sid {
        if let Some(user_id) = session_store.get_user_id(&session_id) {
            // Fetch user
            let user: Option<User> = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE id = ?"
            )
            .bind(&user_id)
            .fetch_optional(&**db)
            .await
            .unwrap();

            if let Some(u) = user {
                let tmpl = ProfileTemplate { username: &u.username };
                return tmpl.render().unwrap();
            }
        }
    }
    poem::Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/auth/login")
        .body("Redirecting to login...")
}

#[handler]
pub async fn logout(req: &Request, session_store: Data<&SessionStore>) -> impl IntoResponse {
    if let Some(sid) = get_session_id_from_cookie(req) {
        session_store.remove_session(&sid);
    }
    let clear_cookie = "hackademy_session_id=; Path=/; HttpOnly; Max-Age=0";
    (
        StatusCode::FOUND,
        [("Location", "/"), (SET_COOKIE, clear_cookie)],
        "Logged out!",
    )
}

fn get_session_id_from_cookie(req: &Request) -> Option<String> {
    if let Some(cookie) = req.cookie() {
        cookie.get("hackademy_session_id").map(|s| s.to_string())
    } else {
        None
    }
}
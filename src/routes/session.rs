// routes/session.rs
use poem::{
    handler,
    http::{header::SET_COOKIE, StatusCode},
    IntoResponse, Request,
};

#[handler]
pub async fn set_session(req: &Request) -> impl IntoResponse {
    // You can parse some user info, then set a cookie
    // This is extremely minimal and not secure for production
    let user_id = "user123"; // example
    let resp = format!("Session set for user: {}", user_id);
    let cookie_value = format!("hackademy_session_id={}; Path=/; HttpOnly", user_id);

    (StatusCode::OK, [(SET_COOKIE, cookie_value)], resp)
}

// Later you can read the cookie from subsequent requests
#[handler]
pub async fn get_session(req: &Request) -> impl IntoResponse {
    if let Some(cookie) = req.cookie() {
        if let Some(session_id) = cookie.get("hackademy_session_id") {
            return format!("Session ID: {}", session_id).into_response();
        }
    }
    "No session found.".into_response()
}
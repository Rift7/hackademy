use serde::{Deserialize, Serialize};

// For user auth
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

// For categories
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: String,
    pub title: String,
}

// For subcategories
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subcategory {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub description: Option<String>,
}

// For questions
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: String,
    pub category_id: String,
    pub subcategory_id: Option<String>,
    pub question_text: String,
    pub options: String, // stored as JSON string
    pub correct_answer_idx: i64,
}

impl Question {
    // Helper to parse the 'options' field into a Vec<String>
    pub fn get_options_vec(&self) -> Vec<String> {
        serde_json::from_str(&self.options).unwrap_or_default()
    }
}
use poem::{
    handler,
    web::{Data, Query},
    IntoResponse,
};
use askama::Template;
use sqlx::{Pool, Sqlite};
use crate::models::{Category, Subcategory, Question};

#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CategoryResult {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct SubCategoryResult {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub description: Option<String>,
    pub parent_category_title: String,
}

#[derive(Clone, Debug)]
pub struct QuestionResult {
    pub id: String,
    pub category_id: String,
    pub subcategory_id: Option<String>,
    pub question_text: String,
}

#[derive(Template)]
#[template(path = "search_results.html")]
pub struct SearchResultsTemplate<'a> {
    title: &'a str,
    query: &'a str,
    categories: &'a [CategoryResult],
    subcategories: &'a [SubCategoryResult],
    questions: &'a [QuestionResult],
}

#[handler]
pub async fn search_handler(
    Query(params): Query<SearchParams>,
    db: Data<&Pool<Sqlite>>,
) -> impl IntoResponse {
    let q = match params.q {
        Some(ref s) if !s.trim().is_empty() => s.trim(),
        _ => {
            let tmpl = SearchResultsTemplate {
                title: "Hackademy - Search",
                query: "",
                categories: &[],
                subcategories: &[],
                questions: &[],
            };
            return tmpl.render().unwrap();
        }
    };

    // 1. Categories
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE title LIKE ?"
    )
    .bind(format!("%{}%", q))
    .fetch_all(&**db)
    .await
    .unwrap();
    let category_results: Vec<CategoryResult> = categories
        .iter()
        .map(|c| CategoryResult {
            id: c.id.clone(),
            title: c.title.clone(),
        })
        .collect();

    // 2. Subcategories
    // We'll join categories to get the parent title
    let subcategories = sqlx::query!(
        r#"
        SELECT 
            subcategories.id as id,
            subcategories.category_id as category_id,
            subcategories.title as title,
            subcategories.description as description,
            categories.title as category_title
        FROM subcategories
        JOIN categories ON subcategories.category_id = categories.id
        WHERE subcategories.title LIKE ?
           OR subcategories.description LIKE ?
        "#,
        format!("%{}%", q),
        format!("%{}%", q)
    )
    .fetch_all(&**db)
    .await
    .unwrap();

    let subcategory_results: Vec<SubCategoryResult> = subcategories
        .iter()
        .map(|row| SubCategoryResult {
            id: row.id.clone(),
            category_id: row.category_id.clone(),
            title: row.title.clone(),
            description: row.description.clone(),
            parent_category_title: row.category_title.clone(),
        })
        .collect();

    // 3. Questions
    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM questions WHERE question_text LIKE ?"
    )
    .bind(format!("%{}%", q))
    .fetch_all(&**db)
    .await
    .unwrap();

    let question_results: Vec<QuestionResult> = questions
        .iter()
        .map(|qt| QuestionResult {
            id: qt.id.clone(),
            category_id: qt.category_id.clone(),
            subcategory_id: qt.subcategory_id.clone(),
            question_text: qt.question_text.clone(),
        })
        .collect();

    let tmpl = SearchResultsTemplate {
        title: "Hackademy - Search",
        query: q,
        categories: &category_results,
        subcategories: &subcategory_results,
        questions: &question_results,
    };
    tmpl.render().unwrap()
}
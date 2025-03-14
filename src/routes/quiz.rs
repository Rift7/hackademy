use poem::{handler, web::{Data, Query, Form}, IntoResponse, Request, post};
use askama::Template;
use sqlx::{Pool, Sqlite};
use crate::models::{Question};

#[derive(Template)]
#[template(path = "quiz.html")]
struct QuizTemplate<'a> {
    title: &'a str,
    questions: &'a [Question],
}

#[derive(Template)]
#[template(path = "quiz_results.html")]
struct QuizResultsTemplate<'a> {
    title: &'a str,
    total_questions: usize,
    correct_count: usize,
    feedback: &'a [QuestionFeedback],
}

#[derive(Clone, Debug)]
pub struct QuestionFeedback {
    pub question_text: String,
    pub selected_option: String,
    pub correct_option: String,
    pub is_correct: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuizParams {
    pub category_id: String,
    pub subcategory_id: Option<String>,
}

#[handler]
pub async fn get_quiz(db: Data<&Pool<Sqlite>>, Query(params): Query<QuizParams>) -> impl IntoResponse {
    let mut sql = r#"SELECT * FROM questions WHERE category_id = ?"#.to_string();
    if params.subcategory_id.is_some() {
        sql.push_str(" AND subcategory_id = ?");
    }
    sql.push_str(" ORDER BY id");

    let questions = if let Some(subcat_id) = &params.subcategory_id {
        sqlx::query_as::<_, Question>(&sql)
            .bind(&params.category_id)
            .bind(subcat_id)
            .fetch_all(&**db)
            .await
            .unwrap()
    } else {
        sqlx::query_as::<_, Question>(&sql)
            .bind(&params.category_id)
            .fetch_all(&**db)
            .await
            .unwrap()
    };

    let tmpl = QuizTemplate {
        title: "Hackademy - Quiz",
        questions: &questions,
    };
    tmpl.render().unwrap()
}

// We store form data as question_<qid> -> selected_option_index
#[derive(Debug, serde::Deserialize)]
pub struct QuizSubmission {
    // We'll parse with a generic approach, or direct from the Request
}

#[handler]
pub async fn submit_quiz(req: &Request, db: Data<&Pool<Sqlite>>) -> impl IntoResponse {
    let form = req.form::<std::collections::HashMap<String, String>>().await.unwrap();
    let mut feedback_list = Vec::new();

    for (key, value) in &form {
        if let Some(question_id) = key.strip_prefix("question_") {
            let selected_idx: i64 = value.parse().unwrap_or(-1);

            let question: Option<Question> = sqlx::query_as::<_, Question>(
                "SELECT * FROM questions WHERE id = ?"
            )
            .bind(question_id)
            .fetch_optional(&**db)
            .await
            .unwrap();

            if let Some(q) = question {
                let correct_answer = q.correct_answer_idx;
                let options_vec = q.get_options_vec();
                let is_correct = correct_answer == selected_idx;

                let correct_option = if (correct_answer as usize) < options_vec.len() {
                    options_vec[correct_answer as usize].clone()
                } else {
                    "Unknown".to_string()
                };

                let selected_option = if (selected_idx as usize) < options_vec.len() && selected_idx >= 0 {
                    options_vec[selected_idx as usize].clone()
                } else {
                    "No Answer".to_string()
                };

                feedback_list.push(QuestionFeedback {
                    question_text: q.question_text,
                    selected_option,
                    correct_option,
                    is_correct,
                });
            }
        }
    }

    let total_questions = feedback_list.len();
    let correct_count = feedback_list.iter().filter(|f| f.is_correct).count();

    let tmpl = QuizResultsTemplate {
        title: "Hackademy - Results",
        total_questions,
        correct_count,
        feedback: &feedback_list,
    };
    tmpl.render().unwrap()
}
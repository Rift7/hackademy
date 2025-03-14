use poem::{handler, web::Data, IntoResponse};
use askama::Template;
use sqlx::{Pool, Sqlite};
use crate::models::Category;

#[derive(Template)]
#[template(path = "category_list.html")]
pub struct CategoryListTemplate<'a> {
    title: &'a str,
    categories: &'a [Category],
}

#[handler]
pub async fn get_categories(db: Data<&Pool<Sqlite>>) -> impl IntoResponse {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY title")
        .fetch_all(&**db)
        .await
        .unwrap();

    let tmpl = CategoryListTemplate {
        title: "Hackademy - Categories",
        categories: &categories,
    };
    tmpl.render().unwrap()
}
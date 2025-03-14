use poem::{handler, web::Path, web::Data, IntoResponse};
use askama::Template;
use sqlx::{Pool, Sqlite};
use crate::models::{Category, Subcategory};

#[derive(Template)]
#[template(path = "subcategory_list.html")]
pub struct SubcategoryListTemplate<'a> {
    title: &'a str,
    category_title: &'a str,
    cat_id: &'a str,
    subcategories: &'a [Subcategory],
}

#[handler]
pub async fn get_subcategories(Path(cat_id): Path<String>, db: Data<&Pool<Sqlite>>) -> impl IntoResponse {
    // Fetch category
    let cat: Option<Category> = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(&cat_id)
        .fetch_optional(&**db)
        .await
        .unwrap();

    if let Some(category) = cat {
        // Get subcategories
        let subs = sqlx::query_as::<_, Subcategory>(
            "SELECT * FROM subcategories WHERE category_id = ? ORDER BY title"
        )
        .bind(&cat_id)
        .fetch_all(&**db)
        .await
        .unwrap();

        let tmpl = SubcategoryListTemplate {
            title: "Hackademy - Subcategories",
            category_title: &category.title,
            cat_id: &category.id,
            subcategories: &subs,
        };
        tmpl.render().unwrap()
    } else {
        poem::http::StatusCode::NOT_FOUND
            .with_reason("Category not found")
            .into_response()
    }
}
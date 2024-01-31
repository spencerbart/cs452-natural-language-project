use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::utils::gpt_request;

#[derive(Deserialize, Debug)]
pub struct MyQuery {
    q: String,
}

pub async fn handler(
    Query(params): Query<MyQuery>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let response = sqlx::query(
        format!(
            r#"
        SELECT json_agg(row_to_json(t))
        FROM (
            {}
        ) t;
    "#,
            params.q
        )
        .as_str(),
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })
    .unwrap();

    let row: Value = response.try_get(0).unwrap();
    println!("{:#?}", row);

    let system_message = "You are going to be given some data that is being returned from a postgres database. Summarize the data into something user friendly and explaining what the data is.".to_string();

    match gpt_request(system_message, row.to_string()).await {
        Ok(response) => (StatusCode::OK, response),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "GPT request failed".to_string(),
        ),
    }

    // (StatusCode::OK, "Hello, World!")
}

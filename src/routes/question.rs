use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::utils::gpt_request;

#[derive(Deserialize, Debug)]
pub struct Question {
    q: String,
}

pub async fn handler(
    Query(params): Query<Question>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let response = sqlx::query!(
        r#"
SELECT
    tc.table_schema,
    tc.table_name,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name,
    columns.column_default,
    columns.is_nullable,
    columns.data_type,
    columns.character_maximum_length
FROM
    information_schema.table_constraints AS tc
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
    AND tc.table_schema = kcu.table_schema
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
    AND ccu.table_schema = tc.table_schema
JOIN information_schema.columns AS columns
    ON columns.table_schema = tc.table_schema
    AND columns.table_name = tc.table_name
    AND columns.column_name = kcu.column_name
WHERE
    tc.constraint_type = 'FOREIGN KEY'
    AND tc.table_schema = 'public'
UNION
SELECT
    table_schema,
    table_name,
    column_name,
    NULL AS foreign_table_name,
    NULL AS foreign_column_name,
    column_default,
    is_nullable,
    data_type,
    character_maximum_length
FROM
    information_schema.columns
WHERE
    table_schema = 'public'
    AND table_name != '_sqlx_migrations'
    AND table_name NOT IN (
        SELECT DISTINCT
            tc.table_name
        FROM
            information_schema.table_constraints AS tc
        WHERE
            tc.constraint_type = 'FOREIGN KEY'
            AND tc.table_schema = 'public'
    );
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let system_message =
        format!("You will be given a question about a postgres database. Here is the metadata for all the postgres tables in the database. You will be asked to write a query that answers the question. Only respond with a valid SQL SELECT query. \n\n{:#?}", response);

    println!("Question: {}", params.q);
    println!("System message: {}", system_message);

    match gpt_request(system_message, params.q).await {
        Ok(response) => (StatusCode::OK, response),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "GPT request failed".to_string(),
        ),
    }
}

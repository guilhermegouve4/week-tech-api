use axum::{Json, extract::{State, Path}, http::StatusCode, response::{IntoResponse, Response}};
use sqlx::SqlitePool;
use serde_json::json;
use crate::models::checkin::Checkin;
use crate::middleware::auth::AuthGuard;

pub async fn checkin(
    State(pool): State<SqlitePool>,
    Json(body): Json<Checkin>
) -> Response {
    if let Err(msg) = body.validate_checkin() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response();
    }

    let result =
        sqlx::query("UPDATE registrations SET checked_in = 1 WHERE student_registration = ?")
        .bind(&body.student_registration)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 =>
            (StatusCode::NOT_FOUND, Json(json!({"error": "ra_not_found"}))).into_response(),
        Ok(_) =>
            (StatusCode::OK, Json(json!({"message": "checked_in"}))).into_response(),
        Err(_) =>
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}

pub async fn undo_checkin(
    _guard: AuthGuard,
    State(pool): State<SqlitePool>,
    Path(ra): Path<String>,
) -> Response {
    let result = sqlx::query("UPDATE registrations SET checked_in = 0 WHERE student_registration = ?")
        .bind(ra)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 =>
            (StatusCode::NOT_FOUND, Json(json!({"error": "not_found"}))).into_response(),
        Ok(_) =>
            (StatusCode::OK, Json(json!({"message": "checkin_undone"}))).into_response(),
        Err(_) =>
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}
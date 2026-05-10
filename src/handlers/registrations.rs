use crate::models::registration::{Registration, RegistrationRow};
use crate::middleware::auth::AuthGuard;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::extract::{State, Json, Path};
use sqlx::SqlitePool;
use serde_json::json;


pub async fn register_student(
    State(pool): State<SqlitePool>,
    Json(body): Json<Registration>
    )
    -> Response {

    if let Err(msg) = body.validate_registration() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response();
    }

    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM registrations WHERE student_registration = ?"
    )
    .bind(&body.student_registration)
    .fetch_one(&pool)
    .await;

    match exists {
        Ok(count) if count > 0 =>
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "ra_already_registered"}))).into_response(),
        Err(_) =>
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
        _ => {}
    }

    let result = sqlx::query(
        "INSERT INTO registrations (name, student_registration, course_name, course_period, coffee_break)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&body.name)
    .bind(&body.student_registration)
    .bind(&body.course_name)
    .bind(body.course_period)
    .bind(body.coffee_break as i32)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "registered"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}

pub async fn list_registrations(
    _guard: AuthGuard,
    State(pool): State<SqlitePool>,
) -> Response {
    let result = sqlx::query_as::<_, RegistrationRow>(
        "SELECT name, student_registration, course_name, course_period, coffee_break, checked_in FROM registrations"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => (StatusCode::OK, Json(rows)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}

pub async fn delete_registration(
    _guard: AuthGuard,
    State(pool): State<SqlitePool>,
    Path(ra): Path<String>,
) -> Response {
    let result = sqlx::query("DELETE FROM registrations WHERE student_registration = ?")
        .bind(ra)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 =>
            (StatusCode::NOT_FOUND, Json(json!({"error": "not_found"}))).into_response(),
        Ok(_) =>
            (StatusCode::OK, Json(json!({"message": "deleted"}))).into_response(),
        Err(_) =>
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}
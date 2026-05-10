use sqlx::{Pool, Sqlite};

pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error>{
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS registrations(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            student_registration TEXT NOT NULL,
            course_name TEXT NOT NULL,
            course_period INTEGER NOT NULL,
            coffee_break INTEGER NOT NULL DEFAULT 0,
            checked_in INTEGER NOT NULL DEFAULT 0
            )"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            submitter_name TEXT NOT NULL,
            submitter_registration TEXT NOT NULL,
            project_name TEXT NOT NULL,
            description TEXT NOT NULL
            )"
    )
    .execute(pool)
    .await?;


    Ok(())
}
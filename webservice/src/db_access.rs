use super::errors::MyError;
use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows
        .into_iter()
        .map(|row| Course {
            id: Some(row.id),
            teacher_id: row.teacher_id,
            name: row.name.clone(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
        .collect();

    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let course_row = sqlx::query!(
        r#"select id, teacher_id, name, time from course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course_row) = course_row {
        Ok(Course {
            id: Some(course_row.id),
            teacher_id: course_row.teacher_id,
            name: course_row.name.clone(),
            time: Some(NaiveDateTime::from(course_row.time.unwrap())),
        })
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let result = sqlx::query!(
        r#"insert into course (id, teacher_id, name) values ($1, $2, $3) returning id, teacher_id, name, time"#,
        new_course.id,
        new_course.teacher_id,
        new_course.name,
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        id: Some(result.id),
        teacher_id: result.teacher_id,
        name: result.name.clone(),
        time: Some(NaiveDateTime::from(result.time.unwrap())),
    })
}

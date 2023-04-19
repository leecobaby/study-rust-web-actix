use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course where teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let course_row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let result = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name,description,format,structure,duration,price,language,level) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let result = sqlx::query!(
        r#"DELETE FROM course where teacher_id = $1 and id = $2"#,
        teacher_id,
        id
    )
    .execute(pool)
    .await?;
    
    Ok(format!("Deleted {:?} rows", result))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let curr_course_row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course where teacher_id = $1 and id = $2"#,
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name = if let Some(name) = update_course.name {
        name
    } else {
        curr_course_row.name
    };

    let description = if let Some(description) = update_course.description {
        description
    } else {
        curr_course_row.description.unwrap_or_default()
    };

    let format = if let Some(format) = update_course.format {
        format
    } else {
        curr_course_row.format.unwrap_or_default()
    };

    let structure = if let Some(structure) = update_course.structure {
        structure
    } else {
        curr_course_row.structure.unwrap_or_default()
    };

    let duration = if let Some(duration) = update_course.duration {
        duration
    } else {
        curr_course_row.duration.unwrap_or_default()
    };

    let price = if let Some(price) = update_course.price {
        price
    } else {
        curr_course_row.price.unwrap_or_default()
    };

    let language = if let Some(language) = update_course.language {
        language
    } else {
        curr_course_row.language.unwrap_or_default()
    };

    let level = if let Some(level) = update_course.level {
        level
    } else {
        curr_course_row.level.unwrap_or_default()
    };

    let updated_row = sqlx::query_as!(
        Course,
        r#"UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8 WHERE teacher_id = $9 and id = $10 RETURNING *"#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id
    ).fetch_one(pool)
    .await;

    if let Ok(course) = updated_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

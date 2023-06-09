use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|row| Teacher {
            id: row.id,
            name: row.name.clone().unwrap(),
            picture_url: row.picture_url.clone().unwrap(),
            profile: row.profile.clone().unwrap(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(r#"SELECT * FROM teacher where id = $1"#, teacher_id)
        .fetch_one(pool)
        .await
        .map(|row| Teacher {
            id: row.id,
            name: row.name.clone().unwrap(),
            picture_url: row.picture_url.clone().unwrap(),
            profile: row.profile.clone().unwrap(),
        })
        .map_err(|_| MyError::NotFound("Teacher not found".into()))?;

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let result = sqlx::query!(
        r#"INSERT INTO teacher (name,picture_url,profile) VALUES ($1,$2,$3) RETURNING *"#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .fetch_one(pool)
    .await?;

    Ok(Teacher {
        id: result.id,
        name: result.name.unwrap(),
        picture_url: result.picture_url.unwrap(),
        profile: result.profile.unwrap(),
    })
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT * FROM teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_| MyError::NotFound("Teacher not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name.clone().unwrap()
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url.clone().unwrap()
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile.clone().unwrap()
        },
    };

    let updated_row = sqlx::query!(
        r#"UPDATE teacher SET name = $1, picture_url = $2, profile = $3 WHERE id = $4 RETURNING *"#,
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|row| Teacher {
        id: row.id,
        name: row.name.clone().unwrap(),
        picture_url: row.picture_url.clone().unwrap(),
        profile: row.profile.clone().unwrap(),
    })
    .map_err(|_| MyError::NotFound("Teacher not found".into()))?;

    Ok(updated_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("DELETE FROM teacher where id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_| MyError::NotFound("Unable to delete teacher".into()))?;

    Ok(format!("Deleted {:?} rows", row))
}

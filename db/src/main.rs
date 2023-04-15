use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("--------------db_pool--------------");

    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where id = $1"#,
        1
    )
    .fetch_all(&db_pool)
    .await
    .expect("Failed to insert new course");

    let mut courses_list = vec![];
    for row in course_rows {
        let course = Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        };
        courses_list.push(course);
    }

    println!("courses_list: {:?}", courses_list);

    Ok(())
}

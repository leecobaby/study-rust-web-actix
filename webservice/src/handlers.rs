use super::db_access::*;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use super::models::Course;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    let course = get_course_details_db(&app_state.db, teacher_id, course_id).await;
    HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course: web::Json<Course> = web::Json(Course {
            teacher_id: 1,
            id: Some(3),
            name: String::from("Test Course"),
            time: None,
        });
        let response = new_course(course, app_data).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id = web::Path::from((1,));
        let response = get_courses_for_teacher(app_data, teacher_id).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from((1, 1));
        let response = get_course_detail(app_data, params).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}

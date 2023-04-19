use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
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
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test Course".into(),
            description: Some("Test Description".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let response = post_new_course(course, app_data).await.unwrap();
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
        let teacher_id = web::Path::from(1);
        let response = get_courses_for_teacher(app_data, teacher_id).await.unwrap();
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
        let response = get_course_detail(app_data, params).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // 查询一个不存在的 id
    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from((1, 100));
        let response = get_course_detail(app_data, params).await;
        match response {
            Ok(_) => println!("Something went wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from((1, 2));
        let update_course = web::Json(UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("this is anohter test course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("Chinese".into()),
            level: Some("Intermediate".into()),
        });
        let response = update_course_details(app_data, params, update_course)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from((1, 3));
        let response = delete_course(app_data, params).await.unwrap();
        println!("{:?}", response);
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_data = web::Data::new(AppState {
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from((1, 101));
        let response = delete_course(app_data, params).await;
        match response {
            Ok(_) => println!("Something went wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}

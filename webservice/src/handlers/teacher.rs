use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>,
    new_teacher: web::Json<CreateTeacher>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.into_inner())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, update_teacher.into_inner())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
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
    async fn get_all_teachers_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = AppState {
            db: db_pool,
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
        };
        let resp = get_all_teachers(web::Data::new(app_state)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_detail_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = AppState {
            db: db_pool,
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
        };
        let resp = get_teacher_details(web::Data::new(app_state), web::Path::from(1))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_new_teacher_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = AppState {
            db: db_pool,
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
        };
        let new_teacher = CreateTeacher {
            name: String::from("test"),
            picture_url: String::from("https://img-blog.csdnimg.cn/20200527153605959.png"),
            profile: String::from("A teacher in Machine Learning"),
        };
        let resp = post_new_teacher(web::Data::new(app_state), web::Json(new_teacher))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_teacher_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = AppState {
            db: db_pool,
            health_check_response: String::from(""),
            visit_count: Mutex::new(0),
        };
        let resp = delete_teacher(web::Data::new(app_state), web::Path::from(1))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

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
use chrono::Utc;

pub async fn new_course(course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    let course_vec = app_state.course.lock().unwrap().clone();
    let course_count = course_vec
        .into_iter()
        .filter(|c| c.teacher_id == course.teacher_id)
        .count();
    let new_course = Course {
        teacher_id: course.teacher_id,
        id: Some(course_count + 1),
        name: course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.course.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            id: None,
            name: String::from("Test Course"),
            time: None,
        });
        let app_data = web::Data::new(AppState {
            health_check_response: String::from("I'm healthy"),
            visit_count: Mutex::new(0),
            course: Mutex::new(vec![]),
        });
        let response = new_course(course, app_data).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}

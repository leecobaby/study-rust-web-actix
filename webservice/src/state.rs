use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub course: Mutex<Vec<Course>>,
}

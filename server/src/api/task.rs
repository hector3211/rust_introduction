use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};
//use std::fmt::{Display, Debug};

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String> {
    return Json(task_identifier.into_inner().task_global_id);
}

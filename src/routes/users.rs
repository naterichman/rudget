use crate::models::users::{User, NewUser};
use crate::error::ServerError;
use actix_web::{delete, get, post, web, HttpResponse};
//use serde_json::json;

#[get("/users/{id}"]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/users"]
async fn create(id: web::Json<NewUser>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().finish())
}

#[delete("/users/{id}"]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().finish())
}

pub fn user_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/users");
    config.service(find);
    config.service(create);
//    config.service(update);
    config.service(delete);
}

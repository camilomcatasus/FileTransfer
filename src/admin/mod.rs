use actix_web::{get, web, HttpResponse, post, put, delete};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_management)
        .service(update_user)
        .service(create_user)
        .service(delete_user);
}

#[get("/users")]
async fn user_management() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/user")]
async fn update_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[put("/user")]
async fn create_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[delete("/user/{user_id}")]
async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

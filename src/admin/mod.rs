use actix_web::{get, web, HttpResponse, post, put, delete};
use minijinja::context;

use crate::{models::{Account, AccountRequest}, AppState};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_management)
        .service(update_user)
        .service(create_user)
        .service(delete_user);
}

#[get("/users")]
async fn user_management(app_state: web::Data<AppState>) -> HttpResponse {
    let conn = app_state.pool.get().unwrap();
    let users = Account::get_many(&conn, AccountRequest::default()).unwrap();
    app_state.render_template("admin.html", context! { users => users })
}

#[post("/users")]
async fn update_user() -> HttpResponse {

    HttpResponse::Ok().insert_header(("HX-Reswap", "none")).finish()
}

#[put("/user")]
async fn create_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[delete("/user/{username}")]
async fn delete_user() -> HttpResponse {

    HttpResponse::Ok().finish()
}

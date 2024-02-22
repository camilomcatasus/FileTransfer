use actix_web::{get, web, HttpResponse, post, put, delete};
use minijinja::context;

use crate::{
    utils::{alert_error, alert_success}, 
    models::{Account, AccountRequest}, AppState
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_management)
        .service(update_users)
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
async fn update_users(app_state: web::Data<AppState>) -> HttpResponse {
    let conn = app_state.pool.get().unwrap();



    HttpResponse::Ok().insert_header(("HX-Reswap", "none")).finish()
}

#[put("/user")]
async fn create_user(app_state: web::Data<AppState>, data: web::Json<Account>) -> HttpResponse {
    let conn = app_state.pool.get().unwrap();

    match data.add(&conn) {
        Ok(_) => app_state.render_incomplete(HttpResponse::Ok()
            .insert_header(alert_success(format!("user {} succesfully created", data.username))), 
            "admin/user_row.html",
            context! { user => data }
            ),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[delete("/user/{username}")]
async fn delete_user(app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let conn = app_state.pool.get().unwrap();
    let username = path.into_inner();
    let delete_result = conn.execute("DELETE FROM Account WHERE username = ?1;", rusqlite::params![&username]);
    match delete_result {
        Ok(_) => return HttpResponse::Ok()
            .insert_header(alert_success(format!("{} deleted", &username)))
            .finish(),
        Err(_) => return HttpResponse::InternalServerError()
            .insert_header(alert_error(format!("An issue occured while deleting {}", &username)))
            .finish(),
    }
}

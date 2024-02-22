use std::slice;
use actix_web::{
    get,
    HttpResponse,
    web, HttpRequest
};
use minijinja::context;

use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(display_page);
}

#[get("/files/{tail:.*}")]
async fn display_page(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let full_path = req.path().to_string();
    let file_path = &full_path[7..];

    app_state.render_template("files.html", context! {});
}

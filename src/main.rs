mod models;
mod admin;

use actix_files as fs;
use minijinja::{value::Value, path_loader, Environment};
use actix_web::{
    web::{self, scope}, 
    App, HttpServer, HttpResponse,
    http::header::ContentType,
    
};
use r2d2_sqlite::SqliteConnectionManager;


pub struct AppState {
    env: minijinja::Environment<'static>,
    pub pool: r2d2::Pool<SqliteConnectionManager>
}

impl AppState {
    pub fn render_template(&self, name: &str, ctx: Value) -> HttpResponse {
        let tmpl = self.env.get_template(name).unwrap();
        let rv = tmpl.render(ctx).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rv)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let mut env = Environment::new();
    env.set_loader(path_loader("pages"));

    let manager = SqliteConnectionManager::file("data.sqlite");
    let pool = r2d2::Pool::new(manager).unwrap();
    let state = web::Data::new(AppState { 
        env,
        pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                scope("/admin")
                .configure(admin::config)
            )
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{web, App, HttpServer, Responder};
use serde_derive::Serialize;

use domain::core::api::api_result::ApiResult;

#[derive(Serialize)]
struct User {
    pub name: String,
}

async fn index() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}

async fn login() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}

async fn logout() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/logout", web::post().to(logout)),
                    )
                    .service(web::scope("/users").route("", web::get().to(index))),
            ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

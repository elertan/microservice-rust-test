use actix_web::{web, Responder, Scope};

use domain::core::api::api_result::ApiResult;

use crate::User;

pub mod auth;

pub fn generate_web_scope() -> Scope {
    web::scope("/v1")
        .service(auth::generate_web_scope())
        .service(web::scope("/users").route("", web::get().to(index)))
}

async fn index() -> impl Responder {
    let users = vec![
        User {
            name: "Dennis".to_string(),
        },
        User {
            name: "Mark".to_string(),
        },
    ];

    let api_result = ApiResult::success(users);
    web::Json(api_result)
}

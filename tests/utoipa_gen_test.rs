#![cfg(feature = "actix_extras")]
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
// use utoipa::openapi_spec;
use utoipa::OpenApi;

#[derive(Deserialize)]
struct Foo {
    ids: Vec<i32>,
}

// mod api {
//     use super::*;

/// Delete foo entity
///
/// Delete foo entity by what
#[utoipa::path(
    responses = [
        (200, "success", String),
        (400, "my bad error", u64),
        (404, "vault not found"),
        (500, "internal server error")
    ],
     params = [
        ("ids" = [i32], query, description = "Search foos by ids"),
   ]
)]
#[get("/foo/{_:.*}")]
// #[deprecated = "this is deprecated"]
// web::Path(id): web::Path<i32>
async fn foo_delete(web::Query(foo): web::Query<Foo>) -> impl Responder {
    let ids = foo.ids;
    HttpResponse::Ok().json(json!({ "searched": ids }))
}
// }

#[test]
#[ignore = "this is just a test bed to run macros"]
fn derive_openapi() {
    // use crate::api::__path_foo_delete;
    #[derive(OpenApi, Default)]
    #[openapi(handler_files = [], handlers = [foo_delete])]
    struct ApiDoc;

    println!("{:?}", ApiDoc::openapi().to_json())
}
#![allow(unused)] // for start only
use std::net::SocketAddr;

use crate::model::ModelController;

pub use self::error::{Error, Result};
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await;

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .merge(web::routes_hello::routes_hello())
        .nest("/api", web::routes_tickets::routes(mc.unwrap().clone()))
        .layer(middleware::map_response(main_response_mapp))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapp(res: Response) -> Response {
    println!("-->> {:12} - main response mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

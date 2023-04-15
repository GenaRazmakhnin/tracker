mod metrics_server;
mod posts;
mod schema;
mod shutdown;
mod auth;
mod error;
mod web;
mod model;
mod state;


pub use self::state::{AppState, Pool};
pub use self::error::{Error,Result};
use crate::shutdown::shutdown_signal;
use crate::model::ModelController;


use axum::{response::Html, routing::get, Router, middleware};
use std::net::SocketAddr;
use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolableConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use self::metrics_server::start_metrics_server;
use std::{
    env,
    fmt::{self},
};
use std::time::Duration;
use tower_cookies::CookieManagerLayer;
use crate::web::auth::mw_require_auth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde_json::json;




#[derive(PartialEq, Debug)]
enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            AppEnv::Dev => "dev",
            AppEnv::Prod => "prod",
        };
        write!(f, "{printable}")
    }
}


async fn main_response_mapper(res: Response) -> Response{
    println!("->> {:<12} - main_response_mapper", "HANDLER");
    println!();
    res
}

async fn start_main_server() -> Result<()>{
    let app_env = match env::var("APP_ENV") {
        Ok(v) if v == "prod" => AppEnv::Prod,
        _ => AppEnv::Dev,
    };
    tracing::info!("Running in {app_env} mode");

    if app_env == AppEnv::Dev {
        match dotenvy::dotenv() {
            Ok(path) => tracing::debug!(".env read successfully from {}", path.display()),
            Err(e) => tracing::debug!("Could not load .env file: {e}"),
        };
    }

    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url.clone());
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(10))
        .build(config).await.unwrap();


    if let Err(err) = pool.get().await { panic!("Cannot connect to database - {err}") }

    let state = AppState::new(pool);

    let mc = ModelController::new().await?;


    let routes_apis = web::routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(mw_require_auth));




    let app = Router::new()
        .route("/", get(handler))
        .merge(web::auth::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback(handler_404)
        .route_layer(middleware::from_fn(metrics_server::track_metrics))
        .with_state(state);



    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tracker=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}


async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn handler() -> Html<&'static str> {
    println!("->> {:<12} - root handler", "HANDLER");
    Html("<h1>Hello, World!</h1>")
}


fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
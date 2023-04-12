mod metrics_server;
mod posts;
mod schema;
mod shutdown;
mod auth;

use axum::{response::Html, routing::get, Router, middleware, Json};
use std::net::SocketAddr;
use axum::extract::{State};
use axum::http::{StatusCode};
use axum::response::{IntoResponse};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use self::metrics_server::start_metrics_server;
use self::posts::Post;
use self::posts::show_posts;
use std::{
    env,
    fmt::{self},
};
use std::time::Duration;
use crate::shutdown::shutdown_signal;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use http::{header, request::Parts};


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


type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;


async fn start_main_server() {
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

    let app = Router::new()
        .route("/", get(handler))
        .route("/posts", get(get_posts))
        .fallback(handler_404)
        .route_layer(middleware::from_fn(metrics_server::track_metrics))
        .with_state(pool);


    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
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


async fn get_posts(State(pool): State<Pool>) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let posts = show_posts(pool).await?;
    Ok(Json(posts))
}


async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

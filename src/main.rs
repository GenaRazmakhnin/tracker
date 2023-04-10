mod metrics;
mod db;
mod models;
mod schema;
mod posts;

use axum::{response::Html, routing::get, Router, middleware, Json};
use std::net::SocketAddr;
use std::time::Duration;
use axum::body::Bytes;
use axum::extract::{MatchedPath, State};
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use crate::metrics::start_metrics_server;
use crate::models::Post;
use crate::posts::show_posts;


type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;


async fn start_main_server() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();


    let app = Router::new()
        .route("/", get(handler))
        .fallback(handler_404)
        .route_layer(middleware::from_fn(metrics::track_metrics)).with_state(pool);


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
    let posts = show_posts(poll).await;
    Ok(Json(posts))
}


async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}


fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
use axum_csrf::CsrfConfig;
use sea_orm::DatabaseConnection;
use tera::Tera;
use axum_macros::FromRef;

#[derive(Clone,FromRef)]
pub struct AppState {
    pub(crate) templates: Tera,
    conn: DatabaseConnection,
    csrf: CsrfConfig
}

impl  AppState {
    pub fn new(templates: Tera, conn: DatabaseConnection, csrf: CsrfConfig) -> Self{
        Self{ templates, conn,csrf }
    }
}
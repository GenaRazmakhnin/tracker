use axum_macros::FromRef;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone,FromRef)]
pub struct AppState{
    pool: Pool
}


impl AppState{
    pub(crate) fn new(pool: Pool) -> Self{
        Self{
            pool
        }
    }
}
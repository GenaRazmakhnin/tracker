use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{get, post, delete};
use crate::{AppState, Result};
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::web::auth::Claims;


pub fn routes(mc: ModelController) -> Router<AppState> {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticker(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(
    claims: Claims,
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("{:?}", claims);
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}


async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<String>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
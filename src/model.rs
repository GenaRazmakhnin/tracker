use std::sync::{Arc, Mutex};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default()
        })
    }
}


impl ModelController {
    pub async fn create_ticker(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = Uuid::new_v4();
        let ticket = Ticket {
            id: id.to_string(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>>{
        let store = self.tickets_store.lock().unwrap();
        Ok(store.iter().filter_map(|t| t.clone()).collect())
    }

    pub async fn delete_ticket(&self, id: String) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket_index = store.iter().position(|t| match t {
            Some(tick) => tick.id == id,
            None => false
        });

        match ticket_index {
            Some(index) => Ok(store.get_mut(index).and_then(|t| t.take()).unwrap()),
            None => Err(Error::TicketDeleteFailIdNotFound {id})
        }
    }
}
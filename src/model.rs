use std::sync::{Arc, Mutex};
use crate::{Error, Result};
use serde::{Deserialize,Serialize};


#[derive(Clone,Debug,Serialize)]
pub struct Ticket{
    pub id: String,
    pub cid: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate{
    title: String
}

#[derive(Clone)]
pub struct ModelController{
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

impl ModelController{
    pub async fn new() -> Result<Self>{
        Ok(Self{
            tickets_store: Arc::default()
        })
    }
}


impl ModelController {

}
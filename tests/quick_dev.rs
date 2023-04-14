#![allow(unused)]


use serde_json::json;
use anyhow::Result;

#[tokio::test]
async fn quick_dev()-> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;
    let req_login = hc.do_post("/auth/login", json!({"username": "tracker", "password": "tracker"}));


    req_login.await?.print().await?;

    let req_ticket = hc.do_post("/api/tickets", json!({"title": "title aaa"}));
    req_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}







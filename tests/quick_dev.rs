#![allow(unused)]

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Jen").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn quick_dev2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let login_req = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    login_req.await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn quick_dev3() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let login_req = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcomeDD"
        }),
    );

    login_req.await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn quick_dev4() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let login_req = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    login_req.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tide::prelude::*;
use tide::{Request, Response};

#[derive(Debug, Deserialize)]
struct TestRequest {
    name: String,
    typ: String,
}

// write get request

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: &str = "127.0.0.1:8080";
    let mut app = tide::new();
    app.at("/orders/shoes").post(make_test_request);
    println!("Server listening at port {:?}", &port);
    app.listen(port).await?;
    Ok(())
}

// Post request
async fn make_test_request(mut req: Request<()>) -> tide::Result {
    // let mut db = HashMap::<String, String>::new();
    let TestRequest { name, typ } = req.body_json().await?;
    // db.insert(name, typ);
    // println!("db {:?}", db);
    Ok(format!("Hello this is a test request with a {} and a {}", name, typ).into())
}

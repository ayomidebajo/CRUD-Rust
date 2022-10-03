use tide::prelude::*;
use tide::{Request, Response};

struct TestRequest {
    name: String,
    typ: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: &str = "0.0.0.0:8080";
    let app = tide::new();
    println!("Server listening at port {:?}", &port);
    app.listen(port).await?;
    Ok(())
}

async fn make_test_request(mut req: Request<()>) -> tide::Result {
    let TestRequest { name, typ } = req.body_json().await?;

    Ok(format!(
        "Hello this is a test request with a {} and a {}",
        name, typ
    ))
}

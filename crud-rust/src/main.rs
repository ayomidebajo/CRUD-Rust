use tide::prelude::*;
use tide::{Request, Response};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: &str = "0.0.0.0:8080";
    let app = tide::new();
    println!("Server listening at port {:?}", &port);
    app.listen(port).await?;
    Ok(())
}

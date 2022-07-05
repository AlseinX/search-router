use std::convert::Infallible;

use anyhow::Result;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

mod handler;
mod preset;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    let port = std::env::var("port");
    let port = port
        .as_ref()
        .map_or(Ok(23301), |x| str::parse(x.as_str()))?;
    let addr = ([0, 0, 0, 0], port).into();

    let make_svc =
        make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handler::handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}

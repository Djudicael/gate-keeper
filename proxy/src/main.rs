use std::net::SocketAddr;

use anyhow::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Server, Uri};
use std::sync::Arc; // atomic reference counter

fn mutate_request(req: &mut Request<Body>) -> Result<()> {
    for key in &[
        "content-length",
        "transfer-encoding",
        "accept-encoding",
        "content-encoding",
    ] {
        req.headers_mut().remove(*key);
    }
    let uri = req.uri();
    let uri_string = match uri.query() {
        None => format!("https://httpbin.org{}", uri.path()),
        Some(query) => format!("https://httpbin.org{}?{}", uri.path(), query),
    };
    *req.uri_mut() = uri_string.parse().context("Parsing URI in mutte_request")?;
    // panic!("uri_string: {:?}", uri_string);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let client: Arc<Client<_, hyper::Body>> = Arc::new(client);
    // let url = Uri::from_static("https://httpbin.org/json");
    // let url = Uri::from_static("http://127.0.0.1:3000");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(move |_| {
        let client = Arc::clone(&client);
        async move {
            Ok(service_fn(move |mut req| {
                let client = Arc::clone(&client);
                async move {
                    mutate_request(&mut req);
                    client
                        .request(req)
                        .await
                        .context("Making request to backend server")
                }
            }))
        }
    });

    Server::bind(&addr)
        .serve(make_svc)
        .await
        .context("Running server")?;

    Ok(())
}

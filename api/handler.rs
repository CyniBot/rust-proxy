use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use your_crate_name::get_message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(_: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::Text(get_message()))?)
}

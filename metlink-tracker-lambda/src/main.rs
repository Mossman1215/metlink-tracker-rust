use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(hello)).await?;
    Ok(())
}

async fn hello(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    let body = reqwest::get("https://www.metlink.org.nz/api/v1/ServiceLocation/1")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    Ok(json!({
        "message": "Go Serverless v1.0! Your function executed successfully!"
    }))
}

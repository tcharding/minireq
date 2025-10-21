//! This example demonstrates the `async` feature.

#[tokio::main]
async fn main() -> Result<(), bitreq::Error> {
    let response = bitreq::get("http://httpbin.org/get").send_async().await?;

    println!("Status: {}", response.status_code);
    println!("Body: {}", response.as_str()?);

    Ok(())
}

//! This is a simple example to demonstrate the usage of this library.

fn main() -> Result<(), minireq::Error> {
    let response = minireq::get("http://example.com").send()?;
    let html = response.as_str()?;
    println!("{}", html);
    Ok(())
}

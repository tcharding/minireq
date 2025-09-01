//! WASM example demonstrating extern C function integration
//!
//! This example shows how to use minireq with the `wasm` feature.
//! The actual HTTP requests are performed by JavaScript through extern C functions.

fn main() {
    // Example usage in WASM environment
    let request = minireq::get("https://httpbin.org/get")
        .with_header("User-Agent", "minireq-wasm/0.1.0");

    // In a real WASM environment, this would call out to JavaScript
    // The JavaScript implementation would need to provide:
    // - minireq_wasm_http_request
    // - minireq_wasm_get_status_code
    // - minireq_wasm_get_response_headers

    println!("Request prepared for WASM execution:");
    println!("URL: https://httpbin.org/get");
    println!("Method: GET");
    println!("Headers: User-Agent: minireq-wasm/0.1.0");

    // Note: This won't actually execute in non-WASM environments
    // as the extern C functions aren't implemented
    println!("To run this, compile to WASM and provide JavaScript implementations");
}

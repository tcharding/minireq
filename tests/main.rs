#![cfg(feature = "std")]

extern crate minireq;
mod setup;

use self::setup::*;
use std::io;

#[test]
#[cfg(feature = "rustls")]
fn test_https() {
    // TODO: Implement this locally.
    assert_eq!(
        get_status_code(minireq::get("https://example.com").send()),
        200,
    );
}

#[test]
fn test_timeout_too_low() {
    setup();
    let result = minireq::get(url("/slow_a"))
        .with_body("Q".to_string())
        .with_timeout(1)
        .send();
    assert!(result.is_err());
}

#[test]
fn test_timeout_high_enough() {
    setup();
    let body = get_body(
        minireq::get(url("/slow_a"))
            .with_body("Q".to_string())
            .with_timeout(3)
            .send(),
    );
    assert_eq!(body, "j: Q");
}

#[test]
fn test_headers() {
    setup();
    let body = get_body(
        minireq::get(url("/header_pong"))
            .with_header("Ping", "Qwerty")
            .send(),
    );
    assert_eq!("Qwerty", body);
}

#[test]
fn test_custom_method() {
    use minireq::Method;
    setup();
    let body = get_body(
        minireq::Request::new(Method::Custom("GET".to_string()), url("/a"))
            .with_body("Q")
            .send(),
    );
    assert_eq!("j: Q", body);
}

#[test]
fn test_get() {
    setup();
    let body = get_body(minireq::get(url("/a")).with_body("Q").send());
    assert_eq!(body, "j: Q");
}

#[test]
fn test_redirect_get() {
    setup();
    let body = get_body(minireq::get(url("/redirect")).with_body("Q").send());
    assert_eq!(body, "j: Q");
}

#[test]
fn test_redirect_post() {
    setup();
    // POSTing to /redirect should return a 303, which means we should
    // make a GET request to the given location. This test relies on
    // the fact that the test server only responds to GET requests on
    // the /a path.
    let body = get_body(minireq::post(url("/redirect")).with_body("Q").send());
    assert_eq!(body, "j: Q");
}

#[test]
fn test_redirect_with_fragment() {
    setup();
    let original_url = url("/redirect#foo");
    let res = minireq::get(original_url).send().unwrap();
    // Fragment should stay the same, otherwise redirected
    assert_eq!(res.url.as_str(), url("/a#foo"));
}

#[test]
fn test_redirect_with_overridden_fragment() {
    setup();
    let original_url = url("/redirect-baz#foo");
    let res = minireq::get(original_url).send().unwrap();
    // This redirect should provide its own fragment, overriding the initial one
    assert_eq!(res.url.as_str(), url("/a#baz"));
}

#[test]
fn test_infinite_redirect() {
    setup();
    let body = minireq::get(url("/infiniteredirect")).send();
    assert!(body.is_err());
}

#[test]
fn test_relative_redirect_get() {
    setup();
    let body = get_body(minireq::get(url("/relativeredirect")).with_body("Q").send());
    assert_eq!(body, "j: Q");
}

#[test]
fn test_head() {
    setup();
    assert_eq!(get_status_code(minireq::head(url("/b")).send()), 418);
}

#[test]
fn test_post() {
    setup();
    let body = get_body(minireq::post(url("/c")).with_body("E").send());
    assert_eq!(body, "l: E");
}

#[test]
fn test_put() {
    setup();
    let body = get_body(minireq::put(url("/d")).with_body("R").send());
    assert_eq!(body, "m: R");
}

#[test]
fn test_delete() {
    setup();
    assert_eq!(get_body(minireq::delete(url("/e")).send()), "n: ");
}

#[test]
fn test_trace() {
    setup();
    assert_eq!(get_body(minireq::trace(url("/f")).send()), "o: ");
}

#[test]
fn test_options() {
    setup();
    let body = get_body(minireq::options(url("/g")).with_body("U").send());
    assert_eq!(body, "p: U");
}

#[test]
fn test_connect() {
    setup();
    let body = get_body(minireq::connect(url("/h")).with_body("I").send());
    assert_eq!(body, "q: I");
}

#[test]
fn test_patch() {
    setup();
    let body = get_body(minireq::patch(url("/i")).with_body("O").send());
    assert_eq!(body, "r: O");
}

#[test]
fn tcp_connect_timeout() {
    let _listener = std::net::TcpListener::bind("127.0.0.1:32162").unwrap();
    let resp = minireq::Request::new(minireq::Method::Get, "http://127.0.0.1:32162")
        .with_timeout(1)
        .send();
    assert!(resp.is_err());
    if let Some(minireq::Error::IoError(err)) = resp.err() {
        assert_eq!(err.kind(), io::ErrorKind::TimedOut);
    } else {
        panic!("timeout test request did not return an error");
    }
}

#[test]
fn test_header_cap() {
    setup();
    let body = minireq::get(url("/long_header"))
        .with_max_headers_size(999)
        .send();
    assert!(body.is_err());
    assert!(matches!(body.err(), Some(minireq::Error::HeadersOverflow)));

    let body = minireq::get(url("/long_header"))
        .with_max_headers_size(1500)
        .send();
    assert!(body.is_ok());
}

#[test]
fn test_status_line_cap() {
    setup();
    let expected_status_line = "HTTP/1.1 203 Non-Authoritative Information";

    let body = minireq::get(url("/long_status_line"))
        .with_max_status_line_length(expected_status_line.len() + 1)
        .send();
    assert!(body.is_err());
    assert!(matches!(
        body.err(),
        Some(minireq::Error::StatusLineOverflow)
    ));

    let body = minireq::get(url("/long_status_line"))
        .with_max_status_line_length(expected_status_line.len() + 2)
        .send();
    assert!(body.is_ok());
}

#[test]
fn test_massive_content_length() {
    setup();
    std::thread::spawn(|| {
        // If minireq trusts Content-Length, this should crash pretty much straight away.
        let _ = minireq::get(url("/massive_content_length")).send();
    });
    std::thread::sleep(std::time::Duration::from_millis(500));
    // If it were to crash, it would have at this point. Pass!
}

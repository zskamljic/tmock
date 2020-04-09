use super::*;

#[test]
fn handle_next_http_normal_appends() {
    let mut output = String::new();
    let mut headers = HttpHeaders::new();
    headers.handle_next_http(&mut output, b'A');

    assert_eq!(1, output.len());
    assert!(matches!(headers.state, State::Status));
    assert!(!headers.carriage_return);
    assert_eq!("A", &output);
}

#[test]
fn handle_next_http_cr() {
    let mut output = String::new();
    let mut headers = HttpHeaders::new();
    headers.handle_next_http(&mut output, 13u8);

    assert!(output.is_empty());
    assert!(matches!(headers.state, State::Status));
    assert!(headers.carriage_return);
}

#[test]
fn handle_next_http_cr_lf() {
    let mut output = "a".to_string();
    let mut headers = HttpHeaders::new();
    headers.carriage_return = true;

    headers.handle_next_http(&mut output, 10u8);

    assert_eq!(1, output.len());
    assert_eq!("a", output);
    assert!(matches!(headers.state, State::HeaderNext));
    assert!(!headers.carriage_return);
}

#[test]
fn handle_next_http_empty() {
    let mut output = String::new();
    let mut headers = HttpHeaders::new();
    headers.state = State::HeaderNext;
    headers.carriage_return = true;

    headers.handle_next_http(&mut output, 10u8);

    assert!(output.is_empty());
    assert!(matches!(headers.state, State::Body));
    assert!(!headers.carriage_return);
}

#[test]
fn handle_next_http_header() {
    let mut output = String::new();
    let mut headers = HttpHeaders::new();
    headers.state = State::HeaderNext;
    headers.carriage_return = true;

    headers.handle_next_http(&mut output, b'A');

    assert_eq!(1, output.len());
    assert!(matches!(headers.state, State::Header));
    assert!(!headers.carriage_return);
}

#[test]
fn handle_http_headers_consumes_all() -> Result<()> {
    let mut buffer = "HTTP/1.1 200 OK\r
Server: nginx\r
Date: Wed, 08 Apr 2020 15:23:09 GMT\r
Transfer-Encoding: chunked\r
Connection: keep-alive\r
\r
"
    .bytes()
    .map(|x| Ok(x));

    let headers = handle_http_headers(&mut buffer)?;

    assert_eq!("HTTP/1.1 200 OK", headers.protocol);
    assert!(headers.headers.contains_key("server"));
    assert!(headers.headers.contains_key("date"));
    assert!(headers.headers.contains_key("transfer-encoding"));
    assert!(headers.headers.contains_key("connection"));

    assert_eq!("nginx", headers.headers["server"]);
    assert_eq!("Wed, 08 Apr 2020 15:23:09 GMT", headers.headers["date"]);
    assert_eq!("chunked", headers.headers["transfer-encoding"]);
    assert_eq!("keep-alive", headers.headers["connection"]);

    assert_eq!(0, buffer.len());
    assert!(matches!(headers.state, State::Body));
    Ok(())
}

use super::*;

#[test]
fn handle_http_body_reads_plain() -> Result<()> {
    let headers = HashMap::new();
    let mut content = "body".bytes().map(|x| Ok(x));

    let result = handle_http_body(&headers, &mut content)?;

    assert_eq!(vec![b'b', b'o', b'd', b'y'], result);

    Ok(())
}

#[test]
fn handle_http_body_reads_at_most_content_length() -> Result<()> {
    let mut headers = HashMap::new();
    headers.insert(CONTENT_LENGTH.to_string(), "4".to_string());
    let mut content = "body123".bytes().map(|x| Ok(x));

    let result = handle_http_body(&headers, &mut content)?;

    assert_eq!(vec![b'b', b'o', b'd', b'y'], result);

    Ok(())
}

#[test]
fn handle_http_chunked_body_works() -> Result<()> {
    let mut input = "4\r
Wiki\r
5\r
pedia\r
E\r
 in\r
\r
chunks.\r
0\r
\r\n"
        .bytes()
        .map(|x| Ok(x));
    let result = handle_http_chunked_body(&mut input)?;
    assert_eq!(b"Wikipedia in\r\n\r\nchunks.".to_vec(), result);

    Ok(())
}

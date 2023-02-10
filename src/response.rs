use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::{configuration::SERVER_DATA_QUERY_DURATION, exchange_api::generate_rate_response};

/// Handles incoming requests to the server.
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = generate_response(&request_line);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn generate_response(request: &String) -> String {
    match &request[..] {
        "GET / HTTP/1.1" => load_html_file("HTTP/1.1 200 OK", "assets/pages/hello.html"),
        "GET /latest HTTP/1.1" => load_exchange_rates(),
        _ => load_html_file("HTTP/1.1 404 NOT FOUND", "assets/pages/404.html"),
    }
}

fn load_exchange_rates() -> String {
    let rate_response = generate_rate_response(SERVER_DATA_QUERY_DURATION);
    let content = serde_json::to_string(&rate_response).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let length = content.len();
    format_response(status_line, length, content)
}

fn load_html_file(status_line: &str, filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format_response(status_line, length, contents);
    response
}

fn format_response(status_line: &str, length: usize, contents: String) -> String {
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

mod test {
    #![allow(dead_code, unused_imports)]
    use super::*;

    #[test]
    fn test_generate_response_home() {
        let mock_request = "GET / HTTP/1.1";
        let file_contents = fs::read_to_string("assets/pages/hello.html").unwrap();
        let length = file_contents.len();
        let expected_response =
            format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{file_contents}");

        assert_eq!(
            expected_response,
            generate_response(&String::from(mock_request))
        );
    }
}

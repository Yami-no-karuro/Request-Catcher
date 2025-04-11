use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

mod line_parser;

fn handle_request(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut request_buffer: [u8; 2048] = [0; 2048];
    stream.read(&mut request_buffer)?;

    let request: String = String::from_utf8_lossy(&request_buffer[..]).to_string();
    let request_lines: Vec<&str> = line_parser::get_all(&request);
    let request_line_parts: Vec<&str> = line_parser::get_parts(&request_lines[0]);

    let request_method: &str = request_line_parts[0];
    let request_path: &str = request_line_parts[1];

    let mut request_body: String = String::new();
    let mut request_headers: Vec<&str> = Vec::new();
    let mut is_body: bool = false;

    for line in &request_lines[1..] {
        if line.is_empty() {
            is_body = true;
            continue;
        }

        if is_body {
            request_body.push_str(line);
            request_body.push('\n');
        } else {
            request_headers.push(line);
        }
    }

    println!("Path: {}", request_path);
    println!("Method: {}", request_method);
    println!("Headers: {:?}", request_headers);

    if request_method != "GET" {
        println!("Body: {}", request_body);
    }

    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(port) = args.get(1) {
        let address: String = format!("127.0.0.1:{}", port);
        let listener: TcpListener = TcpListener::bind(address).unwrap();
        println!("Listening on port: {}.", port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(move || {
                if let Err(e) = handle_request(stream) {
                    println!("An unexpected error occurred: \"{}\".", e);
                    println!("Unable to handle request.");
                }
            });
        }
    }

    println!("Please, provide a valid port.");
}

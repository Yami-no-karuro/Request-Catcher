use std::env;
use std::thread;
use std::process;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod line_parser;

fn write_log(method: &str, path: &str, headers: &[&str], body: &str) -> Result<(), io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("requests.log.txt")?;

    writeln!(file, "------")?;
    writeln!(file, "Request: [{}] - {}", method, path)?;
    writeln!(file, "Headers: \n[")?;
    for header in headers {
        writeln!(file, "  {}", header)?;
    }

    writeln!(file, "]")?;

    if "GET" != method {
        write!(file, "Body: \n{}", &body[..])?;
    }

    writeln!(file, "------")?;
    return Ok(());
}

fn handle_request(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut request_buffer: [u8; 4096] = [0; 4096];
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

    println!("[{}] - {}", request_method, request_path);

    let _ = write_log(request_method, request_path, &request_headers, &request_body);
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: \"Invalid arguments!\"");
        eprintln!("Usage: \"{} <port>\"", args[0]);
        process::exit(1);
    }

    let port: &str = &args[1];
    let address: String = format!("127.0.0.1:{}", port);
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    println!("Request catcher listening on port: {}.", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            if let Err(e) = handle_request(stream) {
                eprintln!("Error: \"{}\".", e);
            }
        });
    }
}

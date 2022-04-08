use std::fs;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    start_server().expect("Something went wrong :(")
}

fn start_server() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        handle(stream?)?;
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let (status, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        (200, "index.html")
    } else {
        (404, "404.html")
    };

    stream.write(http_response(status, fs::read_to_string(filename)?).as_bytes())?;
    stream.flush()
}

fn http_response(status: u32, body: String) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        if status == 200 { "OK" } else { "NOT FOUND" },
        body.len(),
        body
    )
}

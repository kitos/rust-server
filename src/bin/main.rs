use rust_server::ThreadPool;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

const DEFAULT_HOST: &str = "127.0.0.1:7878";

fn main() {
    start_server(None).expect("Something went wrong :(")
}

fn start_server(host: Option<&str>) -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());
    let listener = TcpListener::bind(host.unwrap_or(DEFAULT_HOST))?;

    println!("Server is listening - http://{}", DEFAULT_HOST);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                if let Err(err) = handle(stream) {
                    println!("Failed to handle: {}", err)
                }
            }),
            Err(err) => {
                println!("Failed to process stream: {}", err)
            }
        }
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

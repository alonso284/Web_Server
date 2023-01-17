use web_server::ThreadPool;
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = match ThreadPool::new(4) {
            Ok(p) => p,
            Err(e) => panic!("Pool has not been created successfuly: {}", e),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        

        println!("WORKING!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (filename, status_line) = 
    if buffer.starts_with(get) {
        ("index.html", "HTTP/1.1 202 OK")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("sleeping.html", "HTTP/1.1 202 OK")
    } else {
        ("other.html", "HTTP/1.1 404 NOT FOUND")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {} \r\n\r\n{}", status_line, contents.len(), contents);


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

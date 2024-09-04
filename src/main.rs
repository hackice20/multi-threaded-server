use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::thread::ThreadId;

fn handle_client(mut stream: TcpStream) {
    let thread_id: ThreadId = thread::current().id();
    println!("Handling connection on thread: {:?}", thread_id);

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    
    thread::sleep(Duration::from_secs(2));

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    println!("Server listening on port 6969...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}


use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::fs;
use std::thread;
use std::time::Duration;
use simple_networking::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024]; //this is a buffer that can store upto 1024 bytes. this buffer stores the data that is read
        stream.read(&mut buffer).unwrap();
        
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    //1. creating a connection - socket creation
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    //2. listening and accepting conection
    println!("Listening to requests on http://127.0.0.1:7878");    
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
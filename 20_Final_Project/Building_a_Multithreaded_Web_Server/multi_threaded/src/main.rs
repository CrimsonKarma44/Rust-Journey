use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multi_threaded::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Starting Connection!");

    let pool = ThreadPool::build(3).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (filename, status) = match request_line.as_str() {
        "GET / HTTP/1.1" => {
            println!("{request_line}");
            ("../static/index.html", "HTTP/1.1 200")
        }
        "GET /sleep HTTP/1.1" => {
            println!("{request_line}");
            thread::sleep(Duration::from_secs(5));
            ("../static/sleep.html", "HTTP/1.1 200")
        }
        _ => {
            println!("{request_line}");
            ("../static/404.html", "HTTP/1.1 404 NOT FOUND")
        }
    };
    let content = fs::read_to_string(filename).unwrap();
    let content_len = content.len();

    let response = format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

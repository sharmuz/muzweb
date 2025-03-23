use muzweb::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Max number of requests served; commencing shutdown.")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, html_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 D'OH!", "404.html"),
    };

    let response = create_response(status_line, html_file);
    stream.write_all(response.as_bytes()).unwrap();
}

fn create_response(status_line: &str, html_file: &str) -> String {
    let contents = fs::read_to_string(html_file).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    response
}

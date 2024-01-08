use std::{fs, thread};
use std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use std::time::Duration;
use async_std::io::{ReadExt, WriteExt};
use async_std::task;
use futures::stream::StreamExt;

fn main() {
    println!("Starting server. thread: {:?}", thread::current().name());


    task::block_on(listen_connections());


    println!("Shutting down.");
}

async fn listen_connections() {
    let listener = match TcpListener::bind("127.0.0.1:7878").await {
        Ok(listener) => listener,
        Err(error) => panic!("Problem starting listener: {:?}", error),
    };
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            // handle_connection(stream).await;
            task::spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .collect();

    let http_request_line = http_request.get(0).unwrap();
    let mut http_response_code = 200;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        http_response_code = 404;
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    println!("Request: {} - {} - {:?}", http_request_line, http_response_code, thread::current().name());
}
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));  //监听tcp链接

    let content = fs::read_to_string("main.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //绑定短裤

    for stream in listener.incoming() {
        handle_client(stream?);  //处理
    }
    Ok(())
}

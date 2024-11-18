use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;
use std::net::TcpListener;
//use std::task::Context;
//use std::io::*;
//use std::


fn main() {

    let listener = TcpListener::bind("127.0.0.1:17878").unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        
        //handle_stream(stream);
        handle_connection(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {

    let mut buffer = [0;1024];
    //stream.read(&mut buffer).unwrap();
    stream.read_exact(&mut buffer);
    
    //编写响应

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //Ok(())
}

//编写响应

fn handle_connection(mut stream: TcpStream) {
    // if let Err(e) = stream.read_exact(&mut buffer) {
    //     eprintln!("Failed to read from stream: {}", e);
    //     return;
    // }
    
    // stream.read_exact(&mut buffer);

    // let contents = fs::read_to_string("hello.html").unwrap();
    // //let response = "HTTP/1.1 200 OK\r\n\r\n";

    // let response = format!(
    //     "HTTP/1.1 200 OK\r\nContent-Lentgh: {}\r\n\r\n{}",
    //     contents.len(),
    //     contents
    // );
    // //stream.write(response.as_bytes()).unwrap();不能一次写入所有的的数据，使用write_all可以确保写入所有数据，分多次写入成功
    // stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

 
        let mut buffer = [0; 1024];
        // 使用 read 方法
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let contents = fs::read_to_string("C:/Users/Surface/Desktop/Code/rust/web_sever/hello/hello.html").unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                }
                stream.flush().unwrap();
            }
            Err(e) => eprintln!("Failed to read from stream: {}", e),
        }
    
    
}
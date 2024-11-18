use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread::{self, Thread};
use std::time::Duration;
use hello::ThreadPool;
//use hello::ThreadPool;
//use threadpool::ThreadPool;
//use std::task::Context;
//use std::io::*;
//use std::


fn main() {

    //创建一个监听端口实例，用于读取流（连接）数据
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //可配置线程数的接口
    //基于线程池创建四个等待工作的线程
    let pool = ThreadPool::new(4);
    //遍历收集流数据转化迭代器对象
    for stream in listener.incoming().take(2) {
        
        //用于接收成功的stream(流)的值
        let stream = stream.unwrap();
        // thread::spawn(||{
        //     handle_connection(stream);
        // }); 
        //对每个线程发送消息，通知等待进程有数据传入需要处理
        pool.execute(|| {
            handle_connection(stream);
        });
        //每次处理完结都关闭正在使用的线程
        println!("Shutting down!");
    //handle_connection();       
    }
}
#[warn(dead_code)]
fn handle_stream(mut stream: TcpStream) {

    let mut buffer = [0;1024];
    //stream.read(&mut buffer).unwrap();
    stream.read_exact(&mut buffer);
    //编写响应
    //let response = "HTTP/1.1 200 OK\r\n\r\n";
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //Ok(())
}

//编写响应

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    //增加请求响应验证

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (state, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "E://project//rust//rust-learn//web_sever//hello//html//hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "E://project//rust//rust-learn//web_sever//hello//html//hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "E://project//rust//rust-learn//web_sever//hello//html//404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lentgh: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    //写入响应
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
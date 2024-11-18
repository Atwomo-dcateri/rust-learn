# code - record

### 服务器响应连接

---
P/1.1 200 OK\r\nContent-Lentgh: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        //写入响应
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {

        
    }

}

---
use std::{
    net::TcpStream,
    io::{Read, Write},
    str::from_utf8
};

fn query_time() {
    match TcpStream::connect("localhost:37000") {
        Ok(mut stream) => {
            stream.write_all(b"").unwrap();
            let mut data = Vec::new();
            match stream.read_to_end(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("{}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn main() {
    query_time();
}

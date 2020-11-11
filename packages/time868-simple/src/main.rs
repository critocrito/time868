use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

fn handle_connection(mut stream: TcpStream) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let response = n.as_secs().to_string();
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        _ => stream.flush().unwrap(),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:37000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())
}

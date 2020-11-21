use pico_args::Arguments;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    process::exit,
    str::from_utf8,
    thread::spawn,
};

struct Args {
    help: bool,
    server: SocketAddr,
    count: u32,
    threads: u32,
}

fn parse_socket_addr(s: &str) -> Result<SocketAddr, &'static str> {
    match s.parse() {
        Ok(socket_addr) => Ok(socket_addr),
        Err(_) => Err("Failed to bind to socket."),
    }
}

pub const HELP: &str = r#"Benchmark RFC 868 implementations

USAGE:
    time868-bench [OPTIONS]

OPTIONS:
    -s, --server SOCKET           Test server at specified socket address.
                                  Defaults to 127.0.0.1:37000.
    -c, --count NUMBER            Set the number of requests to send.
                                  Defaults to 1000.
    -t, --threads NUMBER          Set the number of threads to increase parallel
                                  requests. Each threads makes <COUNT> requests.
                                  Defaults to 1.
    -h, --help                    Prints help information.
"#;

fn cli_args() -> Result<Args, pico_args::Error> {
    let mut args = Arguments::from_env();
    Ok(Args {
        help: args.contains(["-h", "--help"]),
        server: args
            .value_from_fn(["-s", "--server"], parse_socket_addr)
            .unwrap_or_else(|_| "127.0.0.1:37000".parse().unwrap()),
        count: args.opt_value_from_str(["-c", "--count"])?.unwrap_or(1000),
        threads: args.opt_value_from_str(["-t", "--threads"])?.unwrap_or(1),
    })
}

fn query_time(server: SocketAddr, count: u32) {
    for _ in 0..count {
        match TcpStream::connect(server) {
            Ok(mut stream) => {
                stream.write_all(b"").unwrap();
                let mut data = Vec::new();
                match stream.read_to_end(&mut data) {
                    Ok(_) => {
                        let _text = from_utf8(&data).unwrap();
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}

fn main() {
    let args = cli_args().unwrap();

    if let true = args.help {
        println!("{}", HELP);
        exit(0);
    }

    let count = if args.count < 1 { 1 } else { args.count };
    let threads = if args.threads < 1 { 1 } else { args.threads };

    println!(
        "Testing server at {:?} ({} requests/{} threads)",
        args.server, count, threads
    );

    let mut thread_handles = vec![];
    for _ in 0..threads {
        let server = args.server.clone();
        thread_handles.push(spawn(move || query_time(server, count)));
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

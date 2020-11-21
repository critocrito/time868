use pico_args::Arguments;
use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    process::exit,
    time::SystemTime,
};

struct Args {
    help: bool,
    listen: SocketAddr,
}

fn parse_socket_addr(s: &str) -> Result<SocketAddr, &'static str> {
    match s.parse() {
        Ok(socket_addr) => Ok(socket_addr),
        Err(_) => Err("Failed to bind to socket."),
    }
}

pub const HELP: &str = r#"A RFC868 time server.

USAGE:
    time868-simple [OPTIONS]

OPTIONS:
    -l, --listen SOCKET           Bind to specified socket address. Defaults to
                                  127.0.0.1:37000.
    -h, --help                    Prints help information.
"#;

fn cli_args() -> Result<Args, pico_args::Error> {
    let mut args = Arguments::from_env();
    Ok(Args {
        help: args.contains(["-h", "--help"]),
        listen: args
            .value_from_fn(["-l", "--listen"], parse_socket_addr)
            .unwrap_or_else(|_| "127.0.0.1:37000".parse().unwrap()),
    })
}

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
    let args = cli_args().unwrap();

    if let true = args.help {
        println!("{}", HELP);
        exit(0);
    }

    println!("Listening server on {:?}.", args.listen);

    let listener = TcpListener::bind(args.listen).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())
}

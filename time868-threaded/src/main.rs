use pico_args::Arguments;
use rayon::ThreadPoolBuilder;
use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    process::exit,
    time::SystemTime,
};

struct Args {
    help: bool,
    listen: SocketAddr,
    threads: usize,
}

fn parse_socket_addr(s: &str) -> Result<SocketAddr, &'static str> {
    match s.parse() {
        Ok(socket_addr) => Ok(socket_addr),
        Err(_) => Err("Failed to bind to socket."),
    }
}

pub const HELP: &str = r#"A RFC868 time server.

USAGE:
    time868-threaded [OPTIONS]

OPTIONS:
    -l, --listen SOCKET           Bind to specified socket address. Defaults to
                                  127.0.0.1:37000.
    -t, --threads                 Set the number of threads to increase parallel
                                  requests. Each threads makes <COUNT> requests.
                                  Defaults to 1.
    -h, --help                    Prints help information.
"#;

fn cli_args() -> Result<Args, pico_args::Error> {
    let mut args = Arguments::from_env();
    Ok(Args {
        help: args.contains(["-h", "--help"]),
        listen: args
            .value_from_fn(["-l", "--listen"], parse_socket_addr)
            .unwrap_or_else(|_| "127.0.0.1:37000".parse().unwrap()),
        threads: args.opt_value_from_str(["-t", "--threads"])?.unwrap_or(1),
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

    let threads = if args.threads < 1 { 1 } else { args.threads };

    println!(
        "Listening server on {:?} ({} threads).",
        args.listen, threads
    );

    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let listener = TcpListener::bind(args.listen).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.spawn(|| handle_connection(stream))
    }

    Ok(())
}

use pico_args::Arguments;
use std::{net::SocketAddr, process::exit, time::SystemTime};
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::*,
    runtime::Builder,
    stream::StreamExt,
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
    time868-async [OPTIONS]

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

async fn handle_connection(mut stream: TcpStream) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let response = n.as_secs().to_string();
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
        _ => stream.flush().await.unwrap(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli_args().unwrap();

    if let true = args.help {
        println!("{}", HELP);
        exit(0);
    }

    let threads = if args.threads < 1 { 1 } else { args.threads };

    let rt = Builder::new_multi_thread()
        .worker_threads(threads)
        .thread_name("time868-async")
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let mut listener = TcpListener::bind(args.listen).await.unwrap();

        println!(
            "Listening server on {:?} ({} threads).",
            args.listen, threads
        );

        while let Some(stream) = listener.next().await {
            let stream = stream.unwrap();
            handle_connection(stream).await;
        }
    });

    Ok(())
}

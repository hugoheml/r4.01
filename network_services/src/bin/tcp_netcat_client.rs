use clap::Parser;
use std::{io::stdin, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    spawn,
};

#[derive(Debug, Parser)]
struct Arguments {
    addr: SocketAddr,
}

async fn real_main() {
    let args = Arguments::parse();

    let sock = TcpStream::connect(args.addr).await.unwrap();
    let (mut rx, mut tx) = sock.into_split();

    spawn(async move {
        let mut buf = String::new();
        while stdin().read_line(&mut buf).is_ok() {
            tx.write_all(buf.as_bytes()).await.unwrap();
            buf.clear();
        }
    });

    let mut buf = [0; 1024];
    loop {
        let len = rx.read(&mut buf).await.unwrap();
        print!("{}", std::str::from_utf8(&buf[..len]).unwrap());
    }
}

#[tokio::main]
async fn main() {
    real_main().await;
}
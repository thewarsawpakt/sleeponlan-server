use std::net::{TcpListener, TcpStream};
use log::{error, info, debug};
use std::io::prelude::*;
use clap::Parser;
use lazy_static_include::lazy_static_include_bytes;
use system_shutdown::force_shutdown;

lazy_static_include_bytes! {
    FILE_SYN => "payloads/syn",
    FILE_ACK => "payloads/ack"
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP to listen on
   #[arg(short, long)]
   ip: String,
    /// Port to listen on
   #[arg(short, long)]
   port: u16,

}

fn parse_input(stream: &mut TcpStream) {
    let mut buf = vec![0u8; 2048];
    let peer = stream.peer_addr().unwrap();
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            debug!("read from stream with address {}", peer);
        },
        Err(_) => {
            error!("failed to read from stream, dropping data from client {}", peer);
            return
        }
    }
    if *FILE_SYN != buf {
        info!("recieved incorrect data from client {}", peer);
        return
    }
    match stream.write(&FILE_ACK) {
        Ok(written) => {
            debug!("wrote {} bytes to client {}", written, peer)
        },
        Err(_) => {
            error!("failed to write 1 ack to client, trying again with address {}", peer)
        }
    }
    loop {
        // We don't need to handle the error, because we'll just keep trying until it succeeds.
        #[allow(unused_must_use)]
        force_shutdown();
    }
}


fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();


    let listener = TcpListener::bind(format!("{}:{}", args.ip, args.port)).expect("Failed to start listening on expected address and port");
    info!("listening for clients on {}:{}", args.ip, args.port);

    for stream in listener.incoming() {
        parse_input(&mut stream?);
    }
    Ok(())
}

extern crate mio;
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;


struct WebSocketServer;

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    println!("Hello, world!");
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
 	let server_socket = TcpListener::bind(&address).unwrap();
	event_loop.register(&server_socket, Token(0), EventSet::readable(), PollOpt::edge()).unwrap();
	event_loop.run(&mut handler).unwrap();
}

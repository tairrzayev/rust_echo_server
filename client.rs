#![feature(io)]
#![feature(env)]
#![feature(net)]

use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::string::String;

fn main() {
	let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();

	let program = args[0].clone();

	if args.len() != 4 {
		print_usage(&program);
		return;
	}

	let address = args[1].clone();
	let port = args[2].clone();
	let message = args[3].clone();

	let connection_string = format!("{}:{}", address, port);

	send_receive_echo_message(&connection_string, &message);
}

fn send_receive_echo_message(addr: &str, message: &str) {
	let mut stream = match TcpStream::connect(addr) {
		Ok(stream) => {
			stream
		}
		Err(e) => {
			println!("Error opening TCP Stream: {}", e);
			return;
		}
	};

	// ignore response to keep this sample code concise.
	// should of course be handled in real world
	let _ = stream.write_all(message.as_bytes());
	let _ = stream.shutdown(Shutdown::Write);

	let mut incoming_data = String::new();
	let _ = stream.read_to_string(&mut incoming_data); // ignore here too
	let _ = stream.shutdown(Shutdown::Read);
	println!("Echo response: {}", incoming_data);
}

fn print_usage(program: &str) {
	println!("Usage: {} <address> <port> <message>", program);
}


use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();

    let program = args[0].clone();

    if args.len() != 3 {
        print_usage(&program);
        return;
    }

    let address = args[1].clone();
    let port = args[2].clone();

    let connection_string = format!("{}:{}", address, port);

    start_server(&connection_string);
}

fn start_server(addr: &str) {
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => {
            listener
        }
        Err(e) => {
            println!("Error creating TCP Connection listener: {}", e);
            return;
        }
    };

    fn handle_client(mut stream: TcpStream) {
        let mut buf = [0u8; 128];
        // we could use read_to_end into vector and then write_all
        // instead of repeated read / writes but creating heap
        // based vector for each connection feels like an overkill
        loop {
            // read some incoming bytes into buffer
            let read_bytes = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(e) => {
                    println!("Error reading the input: {}", e);
                    return;
                }
            };

            // if nothing was read, bail out
            if read_bytes == 0 {
                break;
            }

            // echo incoming bytes back
            match stream.write(&mut buf) {
                Ok(_) => { /* the operation has succeed */ }
                Err(e) => {
                    println!("Error echoing back: {}", e);
                    return;
                }
            };
        }
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
                return;
            }
        }
    }
}

fn print_usage(program: &str) {
    println!("Usage: {} <address> <port>", program);
}


# Rust echo server
Simple rust echo server and its client program, communicating via TCP.  
The server simply echoes all the received data back to the client.

### How to compile
* The sources were compiled using [Rust 1.0.0]
* You will also need `make`
[Rust 1.0.0]: https://github.com/rust-lang/rust/releases/tag/1.0.0

### How to run
* `make` the project:
```sh
$ make
```
* Run the server on some non-privileged port (>= 1024). For example:
```sh
$ ./server 127.0.0.1 2048
```
* Now, send _hello_ to the server:
```sh
$ ./client 127.0.0.1 2048 hello
Echo response: hello
```

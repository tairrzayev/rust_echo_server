RUSTC=rustc

.PHONY: clean

all: server client

client: client.rs 
	$(RUSTC) $<

server: server.rs 
	$(RUSTC) $<

clean:
	rm -f server client

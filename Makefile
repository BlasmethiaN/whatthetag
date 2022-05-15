.PHONY: build install all
build: 
	cargo build --release

install: build
	rm -f ~/.local/bin/wtt
	cp target/release/whatthetag ~/.local/bin/wtt

all: build 


all:
	cargo build --release
	rm -f ~/.local/bin/wtt
	cp target/release/whatthetag ~/.local/bin/wtt

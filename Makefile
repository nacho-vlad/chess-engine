
RUST_SOURCE = $(wildcard ./pychess/src/*.rs)

.PHONY: RUST_SOURCE

.PHONY: bot
bot: RUST_SOURCE
	cargo build --release
	cp ./target/release/engine ./lichess-bot/engines/sah-o-matic

.PHONY: run
run: rustchess.so
	python3 -m app

rustchess.so: librustchess.so
	cp ./target/release/librustchess.so ./app/rustchess.so

librustchess.so: RUST_SOURCE
	cargo build --release


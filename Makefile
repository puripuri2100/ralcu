.PHONY: test build

build:
	llmaker src/parser.mkr
	patch src/parser.rs src/parser.patch
	cargo fmt
	cargo build

test:
	cargo test
	cargo run -- --type file test/1.txt
	cargo run -- --type file test/2.txt
	cargo run -- --type file test/3.txt
	cargo run -- --type file test/4.txt
	cargo run -- --type file test/5.txt
	cargo run -- --type file test/6.txt
	cargo run -- --type file test/7.txt
	cargo run -- --type file test/8.txt
	cargo run -- --type file test/9.txt
	cargo run -- --type file test/10.txt
	cargo run -- --type file test/11.txt
	cargo run -- --type file test/12.txt
	cargo run -- --type file test/13.txt
	cargo run -- --type file test/14.txt
	cargo run -- --type file test/15.txt
	cargo run -- --type file test/16.txt
	cargo run -- --type file test/17.txt
	cargo run -- --type file test/18.txt
	cargo run -- --type file test/19.txt
	cargo run -- --type file test/20.txt
	cargo run -- --type file test/21.txt
	cargo run -- --type file test/22.txt
	cargo run -- --type file test/23.txt
	cargo run -- --type file test/let.txt
	cargo run -- --type file test/letrec.txt

.PHONY: test build

build:
	llmaker src/parser.mkr
	cargo fmt
	cargo build

test:
	cargo test
	target\debug\ralcu.exe test/1.txt
	target\debug\ralcu.exe test/2.txt
	target\debug\ralcu.exe test/3.txt
	target\debug\ralcu.exe test/4.txt
	target\debug\ralcu.exe test/5.txt
	target\debug\ralcu.exe test/6.txt
	target\debug\ralcu.exe test/7.txt
	target\debug\ralcu.exe test/8.txt
	target\debug\ralcu.exe test/9.txt
	target\debug\ralcu.exe test/10.txt
	target\debug\ralcu.exe test/11.txt
	target\debug\ralcu.exe test/12.txt
	target\debug\ralcu.exe test/13.txt
	target\debug\ralcu.exe test/14.txt
	target\debug\ralcu.exe test/15.txt
	target\debug\ralcu.exe test/16.txt
	target\debug\ralcu.exe test/17.txt
	target\debug\ralcu.exe test/18.txt
	target\debug\ralcu.exe test/19.txt
	target\debug\ralcu.exe test/20.txt
	target\debug\ralcu.exe test/21.txt
	target\debug\ralcu.exe test/22.txt
	target\debug\ralcu.exe test/23.txt

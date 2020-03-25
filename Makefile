build:
	cargo build --verbose --all

test: build
	cd tests && ./run.sh

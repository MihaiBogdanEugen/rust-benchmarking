
## help: Prints this help message
help:
	@echo "\nrust-benchmarking\n\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build-debug: Build the local package and all of its dependencies
build-debug:
	cargo build

## run-debug: Build and run the current package
run-debug: build-debug
	RUST_BACKTRACE=full cargo run -- --val $(val)

## build: Build the local package and all of its dependencies with optimizations (release mode)
build:
	cargo build --release

## run: Build and run the current optimized package
run: build
	cargo run --release -- --val $(val)

## bench: Run benchmark for package
bench: build
	cargo bench

## test: Run tests for package
test:
	cargo test

## update: Update dependencies listed in Cargo.lock
update:
	cargo update

## check: Analyze the current package and report errors, but don't build object files
check:
	cargo check --verbose

## clean: Clean the current package and all build artifacts
clean:
	@rm -rdf Cargo.lock && cargo clean

## fmt: Format all Rust files of the current crate
fmt:
	cargo fmt --all

## test-all: Run all the tests
test-all:
	cargo test --verbose

## clippy: Run cargo clippy for static ckecks
clippy:
	cargo clippy --all-targets --all-features --verbose

## update-input: Update git submodule input
update-input:
	git submodule update --remote

.PHONY: help build-debug run-debug build run bench test update check clean fmt test-all clippy update-input

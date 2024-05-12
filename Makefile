# Makefile

# Default target executed when no arguments are given to make.
default: build

# Set the target for Rust's WebAssembly and other initial setup tasks
init:
	rustup target add wasm32-unknown-unknown && cargo install wasm-pack && rustup install nightly && rustup default nightly

# Build the project using wasm-pack
build:
	wasm-pack build --target nodejs

# Clean up the project
clean:
	cargo clean

# PHONY is used to specify that these aren't "real" targets that build files.
.PHONY: default init build clean

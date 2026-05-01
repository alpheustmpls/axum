set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

# Default action
_:
    just --list -u

# Generate HTTPS certificates
cert:
    mkdir -p target/.cert
    mkcert \
    --cert-file target/.cert/cert.pem \
    --key-file target/.cert/key.pem \
    localhost 127.0.0.1 ::1

# Format code
fmt:
    cargo fmt

# Lint code with ls-lint
ls-lint:
    ls-lint -config ./.ls-lint.yaml

# Lint code with ls-lint
lslint:
    just ls-lint

# Lint code with typos-cli
typos:
    typos

# Lint code
lint:
    just lslint
    just typos
    cargo check
    cargo clippy

# Run tests
test:
    cargo test -- --nocapture

# Check code
check:
    just fmt
    just lint
    just test

# Start the dev server with HTTPS
[env("DEV_HTTPS", "1")]
https:
    cargo run --bin app

# Start the dev server with HTTPS
dev:
    just https

# Start the dev server with HTTP
http:
    cargo run --bin app

# Build the project
build:
    cargo build --release

# Start the test server
[env("RUST_ENV", "test")]
start-test:
    ./target/release/app

# Start the production server
[env("RUST_ENV", "production")]
start:
    ./target/release/app

# Clean the project
clean:
    cargo clean

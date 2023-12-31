# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Audits dependencies with cargo deny
audit:
    cargo deny check advisories

# Check the package and deps for errors. i.e. "does it build"
check:
    cargo check

# Formats the source files
format:
    cargo fmt 

# Runs clippy on the sources 
lint:
    cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery

# Run the server locally
run:
    cargo run 

# Watch files for changes, keep the running server up-to-date
start:
    cargo watch \
            -x check \
            -x test \
            -x run

# Runs unit tests
test:
    cargo test

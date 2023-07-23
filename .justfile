# Run an individual test using the Lune CLI
run-test TEST_NAME:
	cargo run -- "tests/{{TEST_NAME}}"

# Run an individual file using the Lune CLI
run-file FILE_NAME:
	cargo run -- "{{FILE_NAME}}"

# Build Lune for release
# TODO: Take input argument for build type, rather than defaulting to `release`
build src/main.rs:
	cargo build --release

# Run tests for the Lune library
test:
	cargo test --lib -- --test-threads 1

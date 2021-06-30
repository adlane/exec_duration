all: release

format:
	@cargo fmt

build:
	@cargo build --release

build_with_serd:
	@cargo build --release --features serde

check:
	@cargo clippy

test:
	@cargo test

doc:
	@cargo doc

release: format check doc build build_with_serd test
	@cargo deny check licenses
	@cargo publish --dry-run

run:
	@cargo run --example hello

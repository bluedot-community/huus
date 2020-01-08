all: force
	cargo build --all-features
	cargo test --all --all-features --no-run

check: force
	cargo check --all-features

test: force
	cargo test --all --all-features -- --nocapture

clean: force
	rm -rf target

force:


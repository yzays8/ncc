.DEFAULT_GOAL := ci-fix

.PHONY: fmt fmt-check lint test ci ci-fix

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

ci:
	$(MAKE) fmt-check
	$(MAKE) lint
	$(MAKE) test

ci-fix:
	$(MAKE) fmt
	$(MAKE) lint
	$(MAKE) test

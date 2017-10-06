.PHONY: help
help:
	@echo	"test           run tests"
	@echo	"lint           run lint"
	@echo	"publish        publish"

.PHONY: test
test: lint
	@cargo test

.PHONY: lint
lint:
	@cargo fmt
	@cargo clippy -- -A unreadable_literal

.PHONY: publish
publish: test
	@cargo publish

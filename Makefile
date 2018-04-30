.PHONY: help
help:
	@echo	"build          run build"
	@echo	"test           run tests"
	@echo	"lint           run lint"
	@echo	"doc            build document"
	@echo	"publish        publish"

.PHONY: build
build: lint
	@cargo build

.PHONY: test
test: lint
	@cargo test

.PHONY: lint
lint:
	@cargo fmt
	@cargo clippy -- -A unreadable_literal

.PHONY: doc
doc:
	@cargo doc -p pinyin --no-deps

.PHONY: publish
publish: test
	@cargo publish

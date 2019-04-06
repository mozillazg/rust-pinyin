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
	@cargo run --example main

.PHONY: lint
lint:
	@cargo run --bin mk_dict > src/dict.rs.new
	@mv src/dict.rs.new src/dict.rs
	@cargo fmt
	@cargo clippy -- -A clippy::unreadable_literal

.PHONY: doc
doc:
	@cargo doc -p pinyin --no-deps

.PHONY: publish
publish: test
	@cargo publish

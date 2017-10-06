.PHONY: help
help:
	@echo	"test           run tests"
	@echo	"lint           run lint"
	@echo	"publish        publish"

.PHONY: test
test:
	@cargo test

.PHONY: lint
lint:
	@cargo fmt

.PHONY: publish
publish: test
	@cargo publish

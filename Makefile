help:
	@echo	"test		run tests"
	@echo	"doc		build document"

test:
	cargo test

doc:
	cargo doc -p pinyin --no-deps

.PHONY: help test doc

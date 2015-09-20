help:
	@echo	"test		run tests"
	@echo	"doc		build document"
	@echo	"publish	publish"

test:
	cargo test

doc:
	cargo doc -p pinyin --no-deps

publish:
	git checkout master && make test && make doc && git checkout master &&\
		cargo publish


.PHONY: help test doc publish

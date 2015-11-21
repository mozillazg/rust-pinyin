.PHONY: help
help:
	@echo	"test			run tests"
	@echo	"doc			build document"
	@echo	"publish		publish"

.PHONY: test
test:
	@cargo test

.PHONY: doc
doc:
	@cargo doc -p pinyin --no-deps

.PHONY: publish
publish:
	@git checkout master && make test &&\
	make doc && cargo publish &&\
	git checkout gh-pages && make doc &&\
	git checkout master

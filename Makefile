NAME := trs
NAME_GEN := trs_gen
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)

.PHONY: oxford_client
oxford_client:
	docker run --rm -v $(PWD):/local swaggerapi/swagger-codegen-cli generate \
			-i https://developer.oxforddictionaries.com/swagger/spec/public_doc_guest.json \
			-l rust \
			-o /local

.PHONY: clean
clean:
	cargo clean


.PHONY: all lambda

lambda:
	cargo build --no-default-features --bin songbook-lambda

deploy-lambda:
	cargo lambda deploy \
		--release \
		--name songbook-lambda \
		--output-format json \
		--package legendary-memory \
		--bin songbook-lambda \
		

.PHONY: all lambda deploy-lambda

lambda:
	cargo build --no-default-features --release --bin songbook-lambda

deploy-lambda: lambda
	cargo lambda deploy \
		--release \
		--name songbook-lambda \
		--output-format json \
		--package legendary-memory \
		--bin songbook-lambda \
		
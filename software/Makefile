
.PHONY: all lambda deploy-lambda

lambda:
	cargo build --no-default-features --release --bin songbook-lambda

deploy-lambda: lambda
	cargo lambda deploy --binary-name songbook-lambda
		
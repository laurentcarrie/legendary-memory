.PHONY: all lambda deploy-lambda build docker

build:
	cargo build --no-default-features --release --bin songbook-lambda

lambda:
	cargo lambda build --no-default-features --release --bin songbook-lambda

deploy-lambda: lambda
	cargo lambda deploy --binary-name songbook-lambda
		

docker:
	aws ecr get-login-password --region eu-west-3 | sudo docker login --username AWS --password-stdin 579871531410.dkr.ecr.eu-west-3.amazonaws.com
	sudo docker build -t laurent/songbook .
	sudo docker tag laurent/songbook:latest 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest
	sudo docker push 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest


run:
	sudo docker run \
		-e AWS_LAMBDA_FUNCTION_NAME=xxx \
		-e AWS_LAMBDA_FUNCTION_MEMORY_SIZE=8000000 \
		-e AWS_LAMBDA_FUNCTION_VERSION=1 \
		-e AWS_LAMBDA_RUNTIME_API=1 \
	       	laurent/songbook 


.PHONY: all lambda deploy-lambda build docker

build:
	cargo build --no-default-features --release --bin songbook-lambda

lambda:
	cargo lambda build --no-default-features --release --bin songbook-lambda

deploy-lambda: lambda
	cargo lambda deploy --binary-name songbook-lambda
		

docker-login:
	aws ecr get-login-password --region eu-west-3 |  docker login --username AWS --password-stdin 579871531410.dkr.ecr.eu-west-3.amazonaws.com

docker-build:  base-image
	# Build the Docker image for the Lambda function
	 docker build -t laurent/songbook -f Dockerfile-2 .
	 docker tag laurent/songbook:latest 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest

docker-push: build lambda docker-build docker-login
	 docker push 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest

base-image : 
	docker build -t mybase -f Dockerfile-3 .

t : 
	docker run --rm -it mybase
	
	

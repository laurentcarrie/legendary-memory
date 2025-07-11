.PHONY: all lambda deploy-lambda build docker base-image

help:
	echo """
	build : build binary
	"""

run:
	cargo build --no-default-features --release --bin songbook-normal
	./target/release/songbook-normal --nb-workers 15 $(realpath ../data/songs) $(realpath ../data/books) $(realpath ../build)


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
	 docker build -t laurent/songbook  --platform=linux/amd64  --provenance=false -f Dockerfile-2 .
	 docker tag laurent/songbook:latest 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest

docker-push: build lambda docker-build docker-login
	 docker push 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/songbook:latest

base-image :
	docker build -t mybase  --platform=linux/amd64 --provenance=false -f Dockerfile-3 .

t : base-image
	docker run --rm -it mybase

t2 :
	docker build -t mybase2   -f Dockerfile-3 .
	docker build -t laurent/songbook2 -f Dockerfile-2 .
	docker run --rm -it laurent/songbook2

lambda-run:
	./target/release/songbook-lambda



check-identity:
	aws sts get-caller-identity --profile default
	aws sts get-caller-identity --profile bayer


batch-run:
	docker build -t laurent/batch -f Dockerfile-batch .
	docker run laurent/batch


batch-push: docker-login
	docker build -t laurent/batch  --platform=linux/amd64  --provenance=false -f Dockerfile-2 .
	docker tag laurent/batch:latest  579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/batch:latest
	docker push 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/batch:latest

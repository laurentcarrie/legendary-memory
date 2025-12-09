.PHONY: all lambda deploy-lambda build docker base-image guitarjpg x gdrive help
.DEFAULT: help

here := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
root := $(dir $(realpath ${here}))
builddir := ${root}build
dataimagesdir := ${root}data/images
# datasongsdir := ${root}data/songs
datasongsdir := ${root}data/songs/the_strokes/the_end_has_no_end
# databooksdir := ${root}data/books
databooksdir := ${root}data/empty

include colorprint


help: ## Show this help message
	@echo "Available commands:"
# 	echo $(MAKEFILE_LIST)
#	was $(MAKEFILE_LIST) but fails if there is an include. So we use Makefile
	@grep -E '^[ a-zA-Z_0-9-]+:.*?## .*$$' Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "$(Red)%-20s$(Color_Off) : $(Blue)%s$(Color_Off)\n", $$1, $$2}'


x:
	echo "here is ${here}"
	echo "root is ${root}"
	echo ${builddir}
	echo "data songsdir is '${datasongsdir}'"
	echo ${dataimagesdir}
	echo ${databooksdir}

guitarjpg:
	mkdir -p ${builddir}/books
	echo "" > logs/songbook.log
	cp ${dataimagesdir}/myguitar.jpg ${builddir}/books/myguitar.jpg



run: build guitarjpg ## run the songbook generation
	mkdir -p ${builddir}
# 	rm -rf ${builddir}/songs/bb_brunes
	echo "" > logs/songbook.log
	./target/release/songbook-cli --force --nb-workers 8 ${datasongsdir} ${databooksdir} ${builddir}

run-y: build guitarjpg ## run the songbook generation
	mkdir -p ${builddir}
	echo "" > logs/songbook.log
	./target/release/songbook-yamake --force --nb-workers 8 ${datasongsdir} ${databooksdir} ${builddir}


run-k: build guitarjpg
	./target/release/songbook-cli --force --nb-workers 12 $(realpath ../data/songs/red_hot_chili_peppers/under_the_bridge) $(realpath ../data/empty) $(realpath ../build)

run-x:
	cargo build --release --bin songbook-cli
	songdir=$(realpath ../data/songs/$x)
	mkdir -p ../data/empty
	bookdir=$(realpath ../data/empty)
	mkdir -p ../build
	builddir=$(realpath ../build)
	rm -rf $builddir
	mkdir -p $builddir
	echo "" > logs/songbook.log
	#./target/release/songbook-cli --force --nb-workers 4 ${songdir} $(bookdir) $(builddir)
	./target/release/songbook-cli --force --nb-workers 4 $(realpath ../data/songs/$x) ../data/empty $(realpath  ../build)




build: ## build the CLI binary
	cargo fmt
	cargo build --release --bin songbook-cli
	cargo build --release --bin songbook-yamake

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


batch-run:
	docker build -t laurent/batch -f Dockerfile-batch .
	docker run laurent/batch


batch-push: docker-login
	docker build -t laurent/batch  --platform=linux/amd64  --provenance=false -f Dockerfile-2 .
	docker tag laurent/batch:latest  579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/batch:latest
	docker push 579871531410.dkr.ecr.eu-west-3.amazonaws.com/laurent/batch:latest

gdrive: run ## deploy to google drive
	rm -rf ${builddir}/delivery/digest 
	${here}/others/shfiles/make_gdrive.sh /zik/songs ${builddir}/delivery 
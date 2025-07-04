#FROM public.ecr.aws/lambda/provided:al2.2025.07.03.10
FROM public.ecr.aws/lambda/provided:al2.2025.07.03.10
#FROM ubuntu


 RUN yum update && yum -y install tar

RUN	apt update
RUN	apt install --fix-missing
RUN	apt install g++ curl -y


RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
RUN	sh rustup.sh -y




RUN	apt update
RUN	apt install --fix-missing
RUN	apt install lilypond -y

COPY target/lambda/songbook-lambda/bootstrap bootstrap
COPY target/lambda/songbook-lambda/bootstrap.zip bootstrap.zip
COPY target/release/songbook-lambda songbook-lambda


CMD [ "./songbook-lambda"  ]

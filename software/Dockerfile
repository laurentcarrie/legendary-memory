#FROM public.ecr.aws/lambda/provided:al2.2025.07.03.10
FROM public.ecr.aws/lambda/provided:al2.2025.07.03.10
#FROM ubuntu


RUN yum -y update && yum -y install tar

RUN yum -y install texlive


# RUN	apt update
# RUN	apt install --fix-missing
# RUN	apt install g++ curl -y


RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
RUN	sh rustup.sh -y



RUN curl --proto '=https' --tlsv1.2 -sSf -L https://gitlab.com/lilypond/lilypond/-/releases/v2.24.4/downloads/lilypond-2.24.4-linux-x86_64.tar.gz  -o lilypond.tar.gz
RUN tar xvzf lilypond.tar.gz
RUN rm lilypond.tar.gz  

COPY target/lambda/songbook-lambda/bootstrap bootstrap
COPY target/lambda/songbook-lambda/bootstrap.zip bootstrap.zip
COPY target/release/songbook-lambda songbook-lambda


CMD [ "./songbook-lambda"  ]

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

#COPY target/release/songbook-lambda ${LAMBDA_TASK_ROOT}/songbook-lambda

ENV PATH="/root/.cargo/bin:${PATH}"

RUN yum install -y gcc openssl-devel musl-dev

RUN	curl --proto '=https' --tlsv1.2 -sSf https://ziglang.org/builds/zig-x86_64-linux-0.15.0-dev.905+edf785db0.tar.xz -o zig.tar.xz

RUN yum -y install xz.x86_64

RUN tar xf zig.tar.xz
RUN rm zig.tar.xz
RUN mv zig-x86_64-linux-0.15.0-dev.905+edf785db0 /usr/local/zig
RUN ln -s /usr/local/zig/zig /usr/local/bin/zig


RUN cargo install cargo-lambda

COPY src src
COPY others others
COPY fonts fonts
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo lambda build --no-default-features --release --bin songbook-lambda

#COPY target/lambda/songbook-lambda/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap
#COPY target/lambda/songbook-lambda/bootstrap.zip ${LAMBDA_RUNTIME_DIR}/bootstrap.zip
#COPY target/release/songbook-lambda songbook-lambda

#COPY target/lambda/songbook-lambda/bootstrap ${LAMBDA_TASK_ROOT}/bootstrap
#COPY target/lambda/songbook-lambda/bootstrap.zip ${LAMBDA_TASK_ROOT}/bootstrap.zip

RUN cp target/lambda/songbook-lambda/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap
#RUN cp target/lambda/songbook-lambda/bootstrap.zip ${LAMBDA_RUNTIME_DIR}/bootstrap.zip
RUN cp target/lambda/songbook-lambda/bootstrap ${LAMBDA_TASK_ROOT}/bootstrap
#RUN cp target/lambda/songbook-lambda/bootstrap.zip ${LAMBDA_TASK_ROOT}/bootstrap.zip

WORKDIR /mnt/efs/tmp


# RUN mkdir -p /opt/bin
# COPY entry.sh /opt/bin/entry.sh
# RUN chmod +x /opt/bin/entry.sh
# ENTRYPOINT [ "/opt/bin/entry.sh" ]
# ENTRYPOINT ["sh"]
#CMD [ "/usr/local/.cargo/bin/songbook-lambda"  ]

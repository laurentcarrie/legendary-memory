# Build stage
FROM rust:1.83-bookworm AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Copy source code
COPY Cargo.toml ./
COPY src ./src

# Build the Lambda binary (generates Cargo.lock if not present)
RUN cargo build --release --target x86_64-unknown-linux-musl --bin band-songbook-lambda

# Runtime stage
FROM public.ecr.aws/lambda/provided:al2023

# Install LaTeX and required tools
RUN dnf update -y && dnf install -y \
    texlive-scheme-full \
    wget \
    tar \
    fontconfig \
    freetype \
    ghostscript \
    && dnf clean all \
    && rm -rf /var/cache/dnf

# Install LilyPond manually (not available in AL2023 repos)
RUN cd /tmp && \
    wget -q https://gitlab.com/lilypond/lilypond/-/releases/v2.24.3/downloads/lilypond-2.24.3-linux-x86_64.tar.gz && \
    tar -xzf lilypond-2.24.3-linux-x86_64.tar.gz && \
    mv lilypond-2.24.3 /opt/lilypond && \
    ln -s /opt/lilypond/bin/lilypond /usr/local/bin/lilypond && \
    rm -f lilypond-2.24.3-linux-x86_64.tar.gz

# Copy the Lambda binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/band-songbook-lambda ${LAMBDA_RUNTIME_DIR}/bootstrap

# Set environment variables
ENV RUST_LOG=info
ENV SRCDIR=s3://zik-laurent/songs
ENV SANDBOX=s3://zik-laurent/sandbox
ENV SETTINGS=s3://zik-laurent/songs/settings.yml

# Lambda handler
CMD [ "bootstrap" ]

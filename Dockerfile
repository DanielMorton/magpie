FROM rust:1.76 as builder
WORKDIR /usr/src/magpie
COPY . .
RUN cargo install --path .

FROM debian:latest
#ARG CGO_ENABLED=0
#RUN apt-get update && apt-get install -y libc6 libc-bin #&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/magpie-bird /usr/local/bin/magpie-bird
ENTRYPOINT ["/usr/local/bin/magpie-bird"]
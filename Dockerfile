FROM rust:latest
COPY . /src
WORKDIR /src
RUN cargo install --path .

FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5
COPY --from=0 /src/target/release/tkn-watch /usr/local/bin/tkn-watch

WORKDIR /
ENTRYPOINT ["/usr/local/bin/tkn-watch"]

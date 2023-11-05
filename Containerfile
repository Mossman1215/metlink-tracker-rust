FROM ghcr.io/chainguard-dev/rust:latest as build

#do build
COPY . /work/
WORKDIR /work/
RUN cargo test
RUN cargo build --release

FROM scratch
COPY --from=build /work/target/release/metlink-tracker-cli /bin/metlink-tracker-cli

CMD ["/bin/metlink-tracker-rust"]
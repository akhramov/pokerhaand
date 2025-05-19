FROM rust:1.86 AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/app/target/release/pokerhaand /usr/local/bin/pokerhaand

EXPOSE 3000
CMD ["pokerhaand"]
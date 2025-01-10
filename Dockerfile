FROM rust:1.84 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update -y
RUN apt install libpq-dev -y
COPY --from=builder /app/target/release/klink /klink
EXPOSE 8000
ENTRYPOINT ["/klink"]
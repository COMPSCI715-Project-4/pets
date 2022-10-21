FROM rust:bullseye AS builder
WORKDIR /app
ADD pets-server .

# build
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/gowalkies /usr/bin/
CMD ["gowalkies"]
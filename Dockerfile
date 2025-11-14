# syntax=docker/dockerfile:1

FROM rust:latest as builder
WORKDIR /app

# Install build tooling.
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk --version 0.16.0 --locked

# Cache dependencies first by copying manifests.
COPY Cargo.toml Cargo.lock Trunk.toml ./
COPY src ./src
COPY static ./static
COPY assets ./assets
COPY index.html .
COPY cards_data.json .
COPY build.rs .

RUN trunk build --release --public-url /

FROM nginx:stable-alpine
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]

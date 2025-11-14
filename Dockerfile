# syntax=docker/dockerfile:1

FROM rust:latest as builder
WORKDIR /app

# Install build tooling.
RUN apt-get update && apt-get install -y curl && \
    rustup target add wasm32-unknown-unknown && \
    curl -L https://github.com/thedodd/trunk/releases/download/v0.20.5/trunk-x86_64-unknown-linux-gnu.tar.gz | \
    tar -xz -C /tmp && \
    mv /tmp/trunk /usr/local/bin/trunk && \
    chmod +x /usr/local/bin/trunk && \
    rm -rf /var/lib/apt/lists/*

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

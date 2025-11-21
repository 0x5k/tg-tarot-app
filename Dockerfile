# syntax=docker/dockerfile:1

FROM rust:latest as builder
WORKDIR /app

# Install build tooling
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk --locked

# Copy project files
COPY Cargo.toml Cargo.lock Trunk.toml ./
COPY src ./src
COPY static ./static
COPY assets ./assets
COPY index.html .
COPY cards_data.json .
COPY build.rs .

# Build the application
RUN trunk build --release --public-url /

# Production stage
FROM nginx:stable-alpine

# Copy built files
COPY --from=builder /app/dist /usr/share/nginx/html

# Add nginx configuration for SPA
RUN echo 'server { \
    listen 8080; \
    server_name _; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    gzip on; \
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml text/javascript; \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]

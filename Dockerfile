# Node.js stage for handling frontend
FROM node:latest as node-stage
WORKDIR /app/frontend
COPY ./frontend/package*.json ./
RUN npm install
COPY ./frontend .
RUN npm run build

# Rust stage for the Rocket.rs server
FROM rust:latest as rust-stage
WORKDIR /app/backend
COPY ./backend .
RUN cargo build --release
RUN ls -la /app/backend/target/release

# Final stage using Debian
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libgcc1 \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary and frontend build
COPY --from=rust-stage /app/backend/target/release/backend /usr/local/bin
COPY --from=rust-stage /app/backend/Rocket.toml /usr/local/bin
COPY --from=node-stage /app/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

EXPOSE 8000

ENTRYPOINT [ "backend"]
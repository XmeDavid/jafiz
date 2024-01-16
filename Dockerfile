FROM --platform=linux/amd64 node:latest as node-stage
WORKDIR /app/frontend
COPY ./frontend/package*.json ./
RUN npm install
COPY ./frontend .
RUN npm run build


FROM --platform=linux/amd64 rust:latest as rust-stage
WORKDIR /app/backend
COPY ./backend .
RUN cargo build --release
RUN ls -la /app/backend/target/release



FROM --platform=linux/amd64 alpine:latest

RUN apk update \
    && apk add build-base libgcc openssl openssl-dev zlib-dev  \
    && rm -rf /var/cache/apk/*


COPY --from=rust-stage /app/backend/target/release/backend /usr/local/bin
COPY --from=node-stage /app/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

EXPOSE 8000

ENTRYPOINT [ "backend", "&&", "ls" ]

FROM node:14.20.1-alpine3.16 as web
WORKDIR /app
COPY web ./web
COPY package.json ./package.json
COPY yarn.lock ./yarn.lock

RUN yarn install
RUN yarn run build

FROM rust:alpine as builder
WORKDIR /
RUN apk update && apk upgrade && apk add --no-cache sqlite
COPY src ./src
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
RUN cargo fetch
RUN cargo build --release

FROM alpine:3.16.2
WORKDIR /app
RUN apk update && apk upgrade && apk add --no-cache sqlite
COPY --from=builder /code/target/release/tonnage /app/tonnage
COPY --from=builder /code/target/release/libdesire.dylib /app/libdesire.dylib
COPY --from=web /app/dist /app/dist
EXPOSE 12306
CMD ["/app/tonnage"]

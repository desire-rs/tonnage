FROM node:14.20.1-alpine3.16 as web
WORKDIR /code
COPY web ./web
COPY package.json ./package.json
COPY yarn.lock ./yarn.lock

RUN yarn install
RUN yarn run build

FROM rust:alpine3.16 as builder
ENV USER=root
WORKDIR /code
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN apk update && apk upgrade --update-cache --available && \
  apk add --no-cache musl-dev && \
  apk add --no-cache openssl && \
  apk add --no-cache libressl-dev && \
  apk add --no-cache libc-dev && \
  apk add --no-cache pkgconfig && \
  apk add --no-cache sqlite-dev
# apk add --no-cache sqlite
RUN cargo init

FROM builder as compiler
WORKDIR /code
COPY src ./src
COPY Cargo.toml ./Cargo.toml
RUN cargo update && cargo fetch
RUN cargo build --release --offline

FROM alpine:3.16.2
WORKDIR /app
RUN apk update && apk upgrade && \
  apk add --no-cache sqlite
COPY --from=compiler /code/target /app/target
COPY --from=web /code/dist /app/dist
COPY env/tonnage.env /app/env/tonnage.env

EXPOSE 12306
ENTRYPOINT [ "target/release/tonnage", "tonnage"]
FROM rust:alpine as build

RUN apk add --no-cache musl-dev

WORKDIR /tmp/build

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine

COPY --from=build /tmp/build/target/x86_64-unknown-linux-musl/release/backend /bin/backend

WORKDIR /

CMD ["backend"]
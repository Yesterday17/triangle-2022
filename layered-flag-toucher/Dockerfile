FROM alpine:3.14

WORKDIR /
COPY ./target/x86_64-unknown-linux-musl/release/layered-flag-toucher /layered-flag-toucher
COPY ./env /env

EXPOSE 8080
CMD /layered-flag-toucher
FROM rustlang/rust:nightly-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["main"]

FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["main"]

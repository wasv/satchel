FROM rustlang/rust:nightly-slim

RUN apt-get update && apt-get install -y default-libmysqlclient-dev \
 && rm -rf /var/lib/apt/lists/*

COPY . /tmp/app
RUN cargo install --path /tmp/app --no-default-features --features mysql && rm -rf /tmp/app

CMD ["/usr/local/cargo/bin/main"]

FROM jdrouet/rust-nightly:buster-slim

WORKDIR /usr/src/ouroboros
COPY . .

RUN cargo install --path .

CMD ["ouroboros"]

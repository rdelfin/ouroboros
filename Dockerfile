FROM rust:1.31

WORKDIR /usr/src/ouroboros
COPY . .

RUN cargo install --path .

CMD ["ouroboros"]

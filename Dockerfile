FROM rustlang/rust:nightly

WORKDIR /usr/src/ouroboros
COPY . .

RUN cargo install --path .

CMD ["ouroboros"]

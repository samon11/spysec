FROM rust:1.66.1

WORKDIR /app

COPY ./ ./

RUN cargo build --release

ENV DATABASE_URL "postgres://postgres:localdb123@pgdb:5432/sec"
CMD ["./target/release/spysec"]
FROM rust:latest

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev && apt-get clean

RUN cargo build --release

CMD ["./target/release/api_gateway"]

FROM rust:latest

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev fish && apt-get clean

RUN useradd -m dariu

USER dariu

SHELL ["/usr/bin/fish", "-c"]

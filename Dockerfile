FROM rust:1.37.0-slim

RUN apt-get update && \
  apt-get install -y vim

WORKDIR /code/

ADD Cargo.lock /code/Cargo.lock
ADD Cargo.toml /code/Cargo.toml

ADD ember-app-boilerplate /code/ember-app-boilerplate
ADD src /code/src
ADD tests /code/tests
ADD . /code/

RUN cargo build

ENTRYPOINT "/bin/bash"

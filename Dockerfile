FROM archlinux

RUN pacman --noconfirm -Sy vim git rust gcc libsass make

WORKDIR /code/

ADD ember-app-boilerplate /code/ember-app-boilerplate

ADD Cargo.lock /code/Cargo.lock
ADD Cargo.toml /code/Cargo.toml

ADD src /code/src
ADD tests /code/tests
ADD . /code/

ENTRYPOINT "/bin/bash"

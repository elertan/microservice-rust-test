FROM rust:1.40

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

# Domain library
#WORKDIR /usr/local/domain

#COPY ./domain/Cargo.* /usr/local/domain/
#RUN mkdir src && touch src/lib.rs && cargo build
COPY ./domain/ /usr/local/domain/
#RUN cargo build

# Application
WORKDIR /usr/local/app

#COPY ./logging_service/Cargo.* /usr/local/app/

# So we can build the app and cache dependencies
#RUN mkdir src
#RUN echo "fn main() {}" > src/main.rs
#RUN cargo build

COPY ./logging_service/ /usr/local/app/

CMD cargo watch -x run

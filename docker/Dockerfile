# Dockerfile to run eportfolio

FROM rust:slim-bullseye

WORKDIR /code

COPY . .

RUN cargo build --release

WORKDIR eportfolio

EXPOSE 8080

CMD ["../target/release/eportfolio"]

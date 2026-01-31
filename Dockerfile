FROM node AS builder

WORKDIR /usr/app

COPY . .

WORKDIR /usr/app/url2ref-web/npm
RUN ./build.sh

# Build mdbook documentation
FROM rust:latest AS mdbook-builder
RUN cargo install mdbook
WORKDIR /usr/app
COPY docs /usr/app/docs
WORKDIR /usr/app/docs
RUN mdbook build

FROM rust:latest

WORKDIR /usr/app

COPY --from=builder /usr/app .
COPY --from=mdbook-builder /usr/app/docs/book /usr/app/docs/book

RUN cargo install --path url2ref-web

WORKDIR /usr/app/url2ref-web

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "url2ref-web" ]


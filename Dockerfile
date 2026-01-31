FROM node AS builder

WORKDIR /usr/app

COPY . .

WORKDIR /usr/app/url2ref-web/npm
RUN ./build.sh

FROM rust:latest

WORKDIR /usr/app

COPY --from=builder /usr/app .

RUN cargo install --path url2ref-web

WORKDIR /usr/app/url2ref-web

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "url2ref-web" ]


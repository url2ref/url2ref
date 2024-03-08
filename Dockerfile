FROM node

WORKDIR /usr/app

COPY . .

WORKDIR /usr/app/url2ref-web/npm
RUN ./build.sh

FROM rust:1.76


COPY --from=0 . .

WORKDIR /usr/app

RUN cargo install --path url2ref-web

WORKDIR /usr/app/url2ref-web

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "cargo", "run", "-r" ]


FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin eno-number
WORKDIR ./eno-number

COPY Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD src ./src

# RUN rm ./target/release/deps/eno-number*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /eno-number/target/release/eno-number ${APP}/eno-number

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./eno-number"]
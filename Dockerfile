FROM rust:slim
RUN mkdir /code
WORKDIR /code
COPY src/ src/
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo install

FROM debian:stretch-slim
COPY --from=0 /usr/local/cargo/bin/process-uwsgi-logs /usr/bin/process-uwsgi-logs
ENTRYPOINT [ "/usr/bin/process-uwsgi-logs" ]

FROM rust
RUN mkdir /code
WORKDIR /code
COPY src/ src/
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo install --path=.
ENTRYPOINT [ "/usr/local/cargo/bin/process-uwsgi-logs" ]

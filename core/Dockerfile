FROM    rust:1.43-slim-stretch as base
RUN     apt -y  update && apt -y install \ 
    pkg-config      \
    clang           \
    build-essential \
    libssl-dev
RUN     rustup update && rustup component add rustfmt 


FROM    base as builder
ADD     . /app
WORKDIR /app/core
RUN     RUST_BACKTRACE=full cargo build --bins -q --release

FROM    debian:stable-slim as server
EXPOSE  7000
ENV     RUST_BACKTRACE=full
RUN     apt -y  update
COPY --from=builder /app/core/target/release/core /app/core
CMD     /app/core
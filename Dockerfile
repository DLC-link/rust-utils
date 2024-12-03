FROM rustlang/rust:nightly-buster AS build

# Build Rust app
COPY ./src /app/dlc-link-utils/src
COPY ./Cargo.toml /app/dlc-link-utils/Cargo.toml

WORKDIR /app

RUN cargo build --release --bin generate-key --target-dir ./dlc-link-utils

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl libpq-dev ca-certificates

COPY --from=build /app/dlc-link-utils/release/generate-key /app/dlc-link-utils/release/generate-key

WORKDIR /app/dlc-link-utils/release/

RUN chmod +x ./generate-key

CMD [ "/bin/bash" ]

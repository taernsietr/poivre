# Build
FROM rust:1.78-slim-bullseye AS build
# FROM rust:alpine AS build
WORKDIR /poivre-axum
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/poivre-axum/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/poivre-axum /bin/server
EOF

# Runtime
FROM debian:bullseye-slim AS final
# FROM alpine:latest AS final
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
USER appuser
COPY --from=build /bin/server /bin/server
EXPOSE 3000
RUN ls /bin/
CMD ["/bin/server"]


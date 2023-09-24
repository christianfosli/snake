FROM rust:1.72 AS builder
ARG CARGO_INSTALL_OPTIONS=''
WORKDIR /app
# Build the service
# - Uses bind mounts so that it is not needed to copy the source code into the container
#   (note that due to using a Cargo workspace the source files for all members members are needed)
# - Uses cache mounts to speed up subsequent builds
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=front-end/src,target=front-end/src \
    --mount=type=bind,source=front-end/Cargo.toml,target=front-end/Cargo.toml \
    --mount=type=bind,source=highscore-api/src,target=highscore-api/src \
    --mount=type=bind,source=highscore-api/Cargo.toml,target=highscore-api/Cargo.toml \
    --mount=type=bind,source=highscore-cleanup-job/src,target=highscore-cleanup-job/src \
    --mount=type=bind,source=highscore-cleanup-job/Cargo.toml,target=highscore-cleanup-job/Cargo.toml \
    --mount=type=bind,source=highscore-types/src,target=highscore-types/src \
    --mount=type=bind,source=highscore-types/Cargo.toml,target=highscore-types/Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cd highscore-cleanup-job && cargo install ${CARGO_INSTALL_OPTIONS} --locked --path .

FROM debian:12-slim AS final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
ARG UID=10001
RUN adduser \
  --disabled-password \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  appuser
USER appuser
COPY --from=builder /usr/local/cargo/bin/highscore-cleanup-job /usr/local/bin/
CMD ["highscore-cleanup-job"]

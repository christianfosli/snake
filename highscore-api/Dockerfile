FROM rust:1.70 AS builder
ARG CARGO_INSTALL_OPTIONS=''
WORKDIR /usr/src/highscore-api
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/src/highscore-api/target \
	cargo install ${CARGO_INSTALL_OPTIONS} --path .

FROM debian:12-slim AS final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/visnake-highscore-api /usr/local/bin/
EXPOSE 3000
CMD ["visnake-highscore-api"]

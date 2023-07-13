FROM rust:1.70 AS builder
# Pass a unique VERSION as build arg to bust browser cache
ARG VERSION=latest 
ARG BUILD_PROFILE=release
ARG HIGHSCORE_API_BASE_URL=""
ENV HIGHSCORE_API_BASE_URL=$HIGHSCORE_API_BASE_URL
RUN curl --fail https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
# Build the app
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
    cd front-end \
    && wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm-${VERSION} --out-dir ./out
WORKDIR /app/front-end
# Copy over static files and add VERSION where useful
COPY front-end/static out/
RUN mv out/style.css out/style-${VERSION}.css && sed -i 's/$VERSION/'"$VERSION"'/g' out/index.html

FROM nginx AS final
COPY ./front-end/nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=builder /app/front-end/out /usr/share/nginx/html
EXPOSE 80

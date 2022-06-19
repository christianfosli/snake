FROM rust:1.61 AS builder
# Pass a unique VERSION as build arg to bust browser cache
ARG VERSION=latest 
ARG BUILD_PROFILE=release
ARG HIGHSCORE_API_BASE_URL=""
ENV HIGHSCORE_API_BASE_URL=$HIGHSCORE_API_BASE_URL
WORKDIR /usr/src/app
RUN curl --fail https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY . .
# Build
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm-${VERSION} --out-dir ./out
# Copy over static files
RUN cp ./static/favicon.ico ./out/
RUN cp ./static/style.css ./out/style-${VERSION}.css
RUN sed 's/$VERSION/'"$VERSION"'/g' static/index.html > ./out/index.html

FROM nginx AS final
COPY ./nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=builder /usr/src/app/out /usr/share/nginx/html
EXPOSE 80

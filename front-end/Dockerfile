FROM rust:1.53

RUN curl --fail https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN curl --fail --location \
  https://github.com/svenstaro/miniserve/releases/download/v0.10.4/miniserve-v0.10.4-linux-x86_64 \
  -o /usr/bin/miniserve && chmod +x /usr/bin/miniserve

WORKDIR /usr/src/app

COPY . .

ARG HIGHSCORE_API_BASE_URL=https://visnakehighscores.azurewebsites.net
ENV HIGHSCORE_API_BASE_URL=$HIGHSCORE_API_BASE_URL
# Pass a unique VERSION as build arg to bust browser cache
ARG VERSION=latest 
ARG BUILD_PROFILE=release

RUN wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm-${VERSION} --out-dir static && \
  mv ./static/style.css ./static/style-${VERSION}.css && \
  sed -i 's/$VERSION/'"$VERSION"'/g' static/index.html

CMD ["miniserve", "./static", "--index", "index.html"]

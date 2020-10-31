FROM rust:1.46

RUN curl --fail https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN curl --fail --location \
  https://github.com/svenstaro/miniserve/releases/download/v0.10.0/miniserve-v0.10.0-linux-x86_64 \
  -o /usr/bin/miniserve && chmod +x /usr/bin/miniserve

WORKDIR /usr/src/app

COPY . .

ARG HIGHSCORE_API_BASE_URL=https://visnakehighscores.azurewebsites.net
ENV HIGHSCORE_API_BASE_URL=$HIGHSCORE_API_BASE_URL
ARG BUILD_PROFILE=release

RUN wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm --out-dir static

CMD ["miniserve", "./static", "--index", "index.html"]

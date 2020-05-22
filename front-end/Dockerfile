FROM node:lts

RUN curl https://sh.rustup.rs -sSf | sh -s --  --default-toolchain stable -y

ENV PATH="/root/.cargo/bin:$PATH"

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY . /src

WORKDIR /src

RUN npm install

EXPOSE 1234

CMD ["npm", "start"]

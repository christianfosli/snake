FROM node:lts

RUN curl https://sh.rustup.rs -sSf | sh -s --  --default-toolchain stable -y
ENV PATH="/root/.cargo/bin:$PATH"

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /usr/src/app

ENV NODE_ENV=development
COPY package*.json ./
RUN npm install

COPY . .

EXPOSE 1234

CMD ["npm", "start"]

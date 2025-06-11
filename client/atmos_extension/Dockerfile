FROM node:latest

COPY . /usr/src/app

WORKDIR /usr/src/app

RUN npm install -g pnpm

RUN rm -rf .git/
RUN rm -rf node_modules

RUN pnpm install

CMD ["pnpm", "build"]

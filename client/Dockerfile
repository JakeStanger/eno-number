# base
FROM node:14 as base
WORKDIR /app
COPY package.json .

# deps
FROM base as deps
RUN yarn install --frozen-lockfile

# release
FROM 
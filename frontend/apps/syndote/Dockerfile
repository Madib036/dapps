FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/syndote ./apps/syndote
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS  \
    REACT_APP_CONTRACT_ADDRESS \
    REACT_APP_SENTRY_DSN
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_CONTRACT_ADDRESS=${REACT_APP_CONTRACT_ADDRESS} \
    REACT_APP_SENTRY_DSN=${REACT_APP_SENTRY_DSN} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/syndote

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/syndote/build"]

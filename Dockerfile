ARG NODE_VERSION=20
ARG PNPM_VERSION=9.4.0
ARG RUST_VERSION=1.79

FROM node:${NODE_VERSION}-alpine as frontend-build

WORKDIR /febuild/frontend

# Enable corepack to access pnpm
RUN corepack enable

# Install dependencies
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

ENV NODE_ENV "production"

# Copy templates for tailwind
WORKDIR /febuild
COPY templates .

WORKDIR /febuild/frontend

# Build assets and place in correct location
COPY frontend .
RUN pnpm build
RUN pnpm gulp copyFiles

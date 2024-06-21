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

FROM rust:${RUST_VERSION}-bookworm AS backend-build

WORKDIR /build

# Install Diesel CLI and copy executable
RUN cargo install cargo-binstall
RUN cargo binstall -y diesel_cli
RUN objcopy --compress-debug-sections /usr/local/cargo/bin/diesel ./diesel

# Copy project files
COPY . .

# Build project into lone executable
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    <<EOF
    set -eux
    cargo build --release
    objcopy --compress-debug-sections target/release/rocket-blogger ./main
EOF

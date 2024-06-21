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

# Copy templates and migrations over to final dir
WORKDIR /build

COPY templates templates
COPY migrations migrations

# Set new workdir (executable will expect files in this workdir in final image)
WORKDIR /app

# Copy project files
COPY . .

# Build project into lone executable
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    <<EOF
    set -eux
    cargo build --release
    objcopy --compress-debug-sections target/release/rocket-blogger /build/main
EOF

FROM debian:bookworm

# Install packages and Diesel CLI
RUN apt update
RUN apt install -y libpq-dev xz-utils curl
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

WORKDIR /app
# Copy required assets from builds
COPY templates templates
COPY --from=frontend-build /febuild/public public
COPY --from=frontend-build /febuild/templates/generated templates/generated
COPY --from=backend-build /build/main ./
COPY --from=backend-build /build/migrations migrations

# Configure Rocket to be correctly exposed outside of container
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# Run migrations and start server
CMD ~/.cargo/bin/diesel migration run && ./main

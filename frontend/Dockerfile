# Production web client. Build context is the kit root (compose: context ., dockerfile frontend/Dockerfile).
# dx compiles the Dioxus crate to wasm, runs Tailwind, and optimizes the bundle; nginx serves it on :5000.
FROM rust:1 AS build

ARG DX_VERSION=0.7.10
RUN rustup target add wasm32-unknown-unknown \
    && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && (cargo binstall -y --force "dioxus-cli@${DX_VERSION}" || cargo install dioxus-cli --version "${DX_VERSION}" --locked)

WORKDIR /build/frontend
# Cargo.toml lists [[test]] targets under ../tests/frontend, so those files must exist.
COPY frontend/ /build/frontend/
COPY tests/frontend/ /build/tests/frontend/

# Baked in at compile time (option_env!). Production points at https://api.<domain>/api/v1.
ARG PUBLIC_API_BASE_URL=/api/v1
ENV PUBLIC_API_BASE_URL=${PUBLIC_API_BASE_URL}

RUN dx bundle --platform web --release --out-dir /build/dist \
    && site="$(dirname "$(find /build/dist -name index.html | head -n 1)")" \
    && cp -r "$site" /build/site

FROM nginx:1-alpine AS production
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /build/site /usr/share/nginx/html
EXPOSE 5000

# 1. Planung & Caching
FROM ghcr.io/koa/rust-cross-compile:0.0.6 as rust
WORKDIR /app

# 2. Builder
FROM rust AS builder
# Installiere leptos toolchain & musl für statisches Linken
RUN rustup target add x86_64-unknown-linux-musl wasm32-unknown-unknown
RUN cargo install cargo-leptos

COPY . .


# Tailwind & Assets bauen und Binary statisch linken
# WICHTIG: LEPTOS_BIN_TARGET_TRIPLE für statisches Bauen der Server-Binary
RUN LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl cargo leptos build --release

# 3. Runtime (Das "Scratch"-ähnliche finale Image)
FROM gcr.io/distroless/static-debian12:latest AS runtime
WORKDIR /app

# Kopiere die statisch gelinkte Binary
# Pfad anpassen: target/x86_64-unknown-linux-musl/release/[PROJEKT_NAME]
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vxlan-provisioner-leptos .

# Kopiere die von Tailwind/Leptos generierten Client-Assets
COPY --from=builder /app/target/site ./site

# Umgebungsvariablen für den Server
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000
CMD ["./vxlan-provisioner-leptos"]
# syntax = docker/dockerfile:1.2

# Using build-kit feature to cache cargo-registry
# (see: https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build)
# NOTE: set -x DOCKER_BUILDKIT 1 (env-variable)

# Load rust and build yew-frontend
FROM rust:latest AS build
WORKDIR /usr/src/front-end
RUN cargo install wasm-pack
COPY ./front-end .
# NOTE: To invalidate cache run:
#  docker builder prune --filter type=exec.cachemount
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/front-end/target \
		wasm-pack build --target web --out-name wasm --out-dir ./static | tee /tmp/wasm-pack-build.txt
CMD ["bash"]

# Serve it with httpd
FROM httpd:latest
COPY --from=build /usr/src/front-end/static /usr/local/apache2/htdocs

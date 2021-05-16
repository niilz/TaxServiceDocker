# Load rust and build yew-frontend
FROM rust:latest AS build
WORKDIR /usr/src/front-end
RUN cargo install wasm-pack
COPY ./front-end .
RUN wasm-pack build --target web --out-name wasm --out-dir ./static | tee /tmp/wasm-pack-build.txt
CMD ["bash"]

# Serve it with httpd
FROM httpd:latest
COPY --from=build /usr/src/front-end/static /usr/local/apache2/htdocs

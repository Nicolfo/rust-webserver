# Rust Webserver
This project aims to provide a WebServer capable of hosting static content, like SPA or static websites. At the moment it doesn't provide https support (connection should be terminated elsewhere, like by an ingress controller) and performance need to be tested.
It's written in Rust and provide a caching mechanism to minimize disk reading. 

# Example of Dockerfile


```
FROM --platform=$BUILDPLATFORM node:22 AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM registry.nicolfo.ovh/nicolfo/rust-webserver:0.0.1-DEBUG

COPY --from=builder /app/dist /usr/local/bin/rust-webserver/static

WORKDIR /usr/local/bin/rust-webserver
EXPOSE 4000
CMD ["./webserver"]
```
available version are 0.0.1 and 0.0.1-DEBUG (the second one adds request logging)

# Building FlareChain Node with Docker

## Dockerfile

\`\`\`dockerfile
FROM rust:latest

WORKDIR /build

RUN apt-get update && apt-get install -y \\
    clang \\
    libclang-dev \\
    libssl-dev \\
    pkg-config \\
    protobuf-compiler

COPY . .

RUN rustup target add wasm32-unknown-unknown && \\
    cargo build --release

FROM ubuntu:22.04

COPY --from=0 /build/target/release/flarechain-node /usr/local/bin/

CMD ["flarechain-node"]
\`\`\`

## Build

\`\`\`bash
cd ~/Desktop/etrid
docker build -t flarechain-node .
\`\`\`

## Extract Binary

\`\`\`bash
docker create --name temp flarechain-node
docker cp temp:/usr/local/bin/flarechain-node ./flarechain-node
docker rm temp
\`\`\`

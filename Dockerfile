FROM ekidd/rust-musl-builder:1.30.1

RUN sudo apt-get update && sudo apt-get install -y libsdl2-dev && sudo rm -rf /var/lib/apt/lists/*

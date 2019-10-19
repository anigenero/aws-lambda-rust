#!/bin/bash

rustup target add x86_64-unknown-linux-musl
brew install filosottile/musl-cross/musl-cross

mkdir .cargo

echo '[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"' > .cargo/config
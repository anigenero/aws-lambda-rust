#!/bin/bash

export OPENSSL_DIR=/usr/local/opt/openssl

# put your variables here
echo "{
  \"RustHandler\": {
    \"foo\": \"bar\"
  }
}" > ./env.vars.json

# put your event variables here
echo "{
  \"input\": \"Hello, World!\"
}" > ./event.json

cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./bootstrap

sam local invoke --template _infra/sam.yml --event ./event.json --env-vars ./env.vars.json
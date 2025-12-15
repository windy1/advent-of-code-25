#!/bin/bash

set -o pipefail

if [ -d "$1" ]; then
    echo "Error: Directory '$1' already exists"
    exit 1
fi

cargo new "$1"
cargo add --package "$1" log env_logger

cat > ./"$1"/src/main.rs << 'EOF'
#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../input_example.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");

fn main() {
    env_logger::builder().format_timestamp(None).init();
    println!("Hello, world!");
}
EOF

jq --arg name "$1" '.configurations = [{
  "name": ("Debug executable " + $name),
  "type": "lldb",
  "request": "launch",
  "env": {
    "RUST_BACKTRACE": "full",
    "RUST_LOG": "debug"
  },
  "cargo": {
    "args": ["run", ("--bin=" + $name), ("--package=" + $name)]
  },
  "args": []
}] + .configurations' .vscode/launch.json > /tmp/launch.json \
    && mv /tmp/launch.json .vscode/launch.json

npx prettier --write .vscode/launch.json

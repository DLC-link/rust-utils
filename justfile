set dotenv-load := true

default:
  just --list

generate-key:
  cargo build --release --bin generate-key --target-dir .
  ./release/generate-key

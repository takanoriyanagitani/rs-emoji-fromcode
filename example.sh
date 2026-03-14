#!/bin/sh

WASM="./rs-emoji-fromcode.wasm"
output="./output.txt"

input(){
  echo radio_button
  echo small_blue_diamond
}

input |
  wasmtime run "${WASM}" |
  cat > "${output}"

file "${output}"

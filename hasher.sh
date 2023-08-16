#!/bin/bash

cd /home/northwood/Desktop/wasm-demo
while true
do
 random-string | wasmtime hasher.wasm
 sleep 1
done

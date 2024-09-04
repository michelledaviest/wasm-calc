#!/usr/bin/bash 

wasm-pack build --release --target nodejs
rm -rf node_modules 
npm i 
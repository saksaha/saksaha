#!/bin/bash
# This is a reference guideline to installing necessary dependencies
# in 'Linux' environment. Use it as a 'reference' and expect inconsistencies
# or any errors.

set -e

echo "Dependency installation script. May require sudo permission"

# Clang
echo "Installing Clang"

sudo apt install clang

# WABT
echo "Installing WABT"

cd ..
cd temp
git clone --recursive https://github.com/WebAssembly/wabt

cd wabt
git submodule update --init
mkdir build

cd build
cmake ..
cmake --build .
sudo cp wasm2wat /usr/bin

cd ../../../
rm -rf temp

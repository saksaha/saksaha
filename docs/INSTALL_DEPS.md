# Installing dependencies

## Clang
```
# Clang
sudo apt install clang
```

## WABT (WebAssembly Binary Toolkit)

- https://github.com/WebAssembly/wabt

```bash
set -e

echo "Dependency installation script. May require sudo permission"


# WABT
git clone --recursive https://github.com/WebAssembly/wabt
cd wabt
git submodule update --init

mkdir build
cd build

cmake ..
cmake --build .
sudo cp wasm2wat /usr/bin
```

## Binaryen

- https://github.com/WebAssembly/binaryen

```
git clone https://github.com/WebAssembly/binaryen.git 
git submodule init
git submodule update
cmake . - DBUILD_STATIC_LIB=ON && make ## Statically link 'libbinaryen.so'!
```


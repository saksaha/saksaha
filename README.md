# Saksaha
Saksaha is a decentralized computing platform. Saksaha aims to provide ways to store and process data in a verifiable yet private way. 

## How to install
### Download the prebuilt binaries
```
...
```

### Build from source

#### Download the source code.
```bash
git clone https://github.com/saksaha/saksaha
```

#### Build
```bash
./ci build
```

## Development
This mono repository contains multiple components that can be developed independent of each other. In order to run `saksaha-network` in development, execute the following in the project directory. Refer to the documentation for more details.
```bash
./ci dev
```

## Test

#### Run all tests
Run all tests. In the project root, execute the following. 
```bash
./ci test
```

#### Run specific tests
Run tests in a specific module or run a single test.
```bash
./ci test --package [package_name] test_name
./ci test [module_name]::
```

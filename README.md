# Saksaha
Saksaha is a decentralized computing platform. Saksaha aims to provide ways to store and process data in a verifiable yet private way. 

## How to install
### Download the prebuilt binaries
Guides will be given later.

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
This mono repository contains multiple components that can be developed 
independent of each other. In order to run `saksaha-network` in development, 
execute the following in the project directory. Refer to the documentation for 
more details.

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
Run tests in a specific module or run a single test. We use `cargo` to build 
and run test cases. Check out its documentation for more detail.

```bash
./ci test --package [package_name] test_name
./ci test [module_name]::
```

e.g. Run all the tests in the package **saksaha_network** under module **node**.
```bash
./ci test --package saksaha_network node::
```

#!/bin/bash

isRustc=$(which rustc)
if [ $? -eq 0 ]; then
    rustcVersion=$(rustc --version)
    printf "Found rustc at: %s, version: %s\n" "$isRustc" "$rustcVersion"
else
    echo "Rustc is not installed, exiting the program"
    exit 1
fi

isCargo=$(which cargo)
if [ $? -eq 0 ]; then
    cargoVersion=$(cargo --version)
    printf "Found cargo at: %s, version: %s\n" "$isCargo" "$cargoVersion"
else
    echo "Cargo is not installed, exiting the program"
    exit 1
fi

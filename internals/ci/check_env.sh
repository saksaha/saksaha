#!/bin/bash

if [[ ! -z "${ROOT_PATH}" ]]; then
    printf "Executing the script from root path: %s\n" "$ROOT_PATH"
else
    echo "Build has to be executed from the project root path"
fi

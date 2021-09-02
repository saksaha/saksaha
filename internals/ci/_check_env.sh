#!/bin/bash

if [[ -z "${ROOT_PATH}" ]]; then
    echo "Build has to be executed from the project root path"
    exit 1
fi

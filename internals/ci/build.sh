#!/bin/bash

set -e

currDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
$currDir/_check_env.sh

cd $ROOT_PATH
cargo build

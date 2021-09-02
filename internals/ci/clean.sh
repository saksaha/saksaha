#!/bin/bash

set -e

currDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

$currDir/check_env.sh
$currDir/check_rust.sh

cd $ROOT_PATH
cargo clean

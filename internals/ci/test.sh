#!/bin/bash

set -e

currDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
$currDir/_check_env.sh

cd $ROOT_PATH
cmd="cargo test -- --nocapture"
# cmd="cargo test -- $@"

echo "Executing: $cmd"

$cmd

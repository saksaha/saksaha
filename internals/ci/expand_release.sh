#!/bin/bash

set -e

currDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
$currDir/_check_env.sh

CI_PROFILE=release $currDir/expand.sh

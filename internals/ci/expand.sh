#!/bin/bash

set -e

curr_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
$curr_dir/_check_env.sh

printf "[ci] Starting expand, CI_PROFILE=%s\n" $CI_PROFILE

cd $ROOT_PATH

# check if rustfmt exists
is_rustfmt=false
if command -v rustfmt &> /dev/null; then
    is_rustfmt=true
    printf "[ci] Found rustfmt, expanded outputs will be formatted\n"
fi

# Clean the destination
if [ "$CI_PROFILE" == "release" ]
then
    expand=$ROOT_PATH/target/expand/release
else
    expand=$ROOT_PATH/target/expand/debug
fi
if [ -d $expand ]; then
    rm -r $expand/*
fi

# Create necessary paths if not present
dests=($expand/bin $expand/lib)
for dst in ${dests[@]}; do
    if [ ! -d $dst ]; then
        printf "[ci] Expand destination path doesn't exist, creating one, \
at: %s\n" \
            $dst
        mkdir -p $dst
    fi
done

# binaries
declare -A bins=(
    [saksaha]=rsak
)

for pkg in "${!bins[@]}"; do
    bin=${bins[$pkg]}
    f=$expand/bin/"$pkg"_"$bin".rs

    printf "[ci] Expand binary, package: %s, binary: %s, to file: %s\n" $pkg $bin $f

    if [ "$CI_PROFILE" == "release" ]
    then
        cargo rustc -p $pkg --bin $bin --profile=check -- -Zunpretty=expanded -C opt-level=3 >> $f
    else
        cargo rustc -p $pkg --bin $bin --profile=check -- -Zunpretty=expanded >> $f
    fi

    if [ $is_rustfmt == "true" ]; then
        rustfmt $f;
    fi
done

# libraries
declare -A libs=(
    [saksaha]=0
)

for pkg in "${!libs[@]}"; do
    f=$expand/lib/"$pkg".rs

    printf "[ci] Expand library, package: %s, to file: %s\n" $pkg $f

    if [ "$CI_PROFILE" == "release" ]
    then
        cargo rustc -p $pkg --lib --profile=check -- -Zunpretty=expanded -C opt-level=3 >> $f
    else
        cargo rustc -p $pkg --lib --profile=check -- -Zunpretty=expanded >> $f
    fi

    if [ $is_rustfmt == "true" ]; then
        rustfmt $f;
    fi
done

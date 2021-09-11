#!/bin/bash

set -e

curr_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
$curr_dir/_check_env.sh

cd $ROOT_PATH

# check if rustfmt exists
is_rustfmt=false
if command -v rustfmt &> /dev/null; then
    is_rustfmt=true
    printf "Found rustfmt, expanded outputs will be formatted\n"
fi

# Clean and check if destination paths exist
expand=$ROOT_PATH/target/expand


if [ -d $expand ]; then
    rm -r $expand/*
fi

dests=($expand/bin $expand/lib)
for dst in ${dests[@]}; do
    if [ ! -d $dst ]; then
        printf "Expand destination path doesn't exist, creating one, at: %s\n" \
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

    printf "Expand binary, package: %s, binary: %s, to file: %s\n" $pkg $bin $f

    cargo rustc -p $pkg --bin $bin --profile=check -- -Zunpretty=expanded >> $f

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

    printf "Expand library, package: %s, to file: %s\n" $pkg $f

    cargo rustc -p $pkg --lib --profile=check -- -Zunpretty=expanded >> $f
    if [ $is_rustfmt == "true" ]; then
        rustfmt $f;
    fi
done

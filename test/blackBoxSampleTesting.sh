#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TEST_DATA_DIR="$SCRIPT_DIR/../data/samples"
files="measurements-1 measurements-10 measurements-10000-unique-keys measurements-2 measurements-20 measurements-3 measurements-boundaries measurements-complex-utf8 measurements-dot measurements-rounding measurements-short measurements-shortest"

for file in $files;
do
    cargo r --quiet -- $TEST_DATA_DIR/$file.txt 2> /dev/null | diff $TEST_DATA_DIR/$file.out -
    if [ $? != 0 ]; then 
        echo "$TEST_DATA_DIR/$file.txt failed."
    fi
done

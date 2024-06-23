#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TEST_DATA_DIR="$SCRIPT_DIR/../data/samples"
cargo r -- $TEST_DATA_DIR/measurements-10.txt 2> /dev/null | diff $TEST_DATA_DIR/measurements-10.out -
#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BASE_URL="https://raw.githubusercontent.com/gunnarmorling/1brc/main"

# Sample 44k row data 
mkdir -p $SCRIPT_DIR/../data/samples
URL="$BASE_URL/data"
wget "$URL/weather_stations.csv" -P $SCRIPT_DIR/../data/

# TODO Change this to a loop that loops over set of strings, and check if the file exists in the data dir first before requesting.
# Sample data for testing
URL="$BASE_URL/src/test/resources/samples"
wget "$URL/measurements-1.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-1.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-10.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-10.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-10000-unique-keys.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-10000-unique-keys.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-2.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-2.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-20.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-20.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-3.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-3.txt" -P $SCRIPT_DIR/../data/samples

wget "$URL/measurements-boundaries.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-boundaries.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-complex-utf8.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-complex-utf8.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-dot.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-dot.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-rounding.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-rounding.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-short.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-short.txt" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-shortest.out" -P $SCRIPT_DIR/../data/samples
wget "$URL/measurements-shortest.txt" -P $SCRIPT_DIR/../data/samples
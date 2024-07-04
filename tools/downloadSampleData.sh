#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BASE_URL="https://raw.githubusercontent.com/gunnarmorling/1brc/main"

# Sample 44k row data 
mkdir -p $SCRIPT_DIR/../data/samples
URL="$BASE_URL/data"
if [ ! -f "$SCRIPT_DIR/../data/weather_stations.csv" ]; then
    wget --quiet "$URL/weather_stations.csv" -P $SCRIPT_DIR/../data/ || echo "Failed to download: $URL/weather_stations.csv"
fi

files="measurements-1 measurements-10 measurements-10000-unique-keys measurements-2 measurements-20 measurements-3 measurements-boundaries measurements-complex-utf8 measurements-dot measurements-rounding measurements-short measurements-shortest"
URL="$BASE_URL/src/test/resources/samples"

# Sample data for testing
for file in $files;
do
    if [ ! -f "$SCRIPT_DIR/../data/samples/$file.out" ]; then
        wget --quiet "$URL/$file.out" -P $SCRIPT_DIR/../data/samples || echo "Failed to download: $URL/$file.out"
    fi
    if [ ! -f "$SCRIPT_DIR/../data/samples/$file.txt" ]; then
        wget --quiet "$URL/$file.txt" -P $SCRIPT_DIR/../data/samples || echo "Failed to download: $URL/$file.txt"
    fi
done

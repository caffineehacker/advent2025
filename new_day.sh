#!/usr/bin/env bash

DAY_OF_MONTH=`date -d tomorrow +%d`
DAY="day$DAY_OF_MONTH"
cp -R template $DAY
cd $DAY
sed -i "s/\"daytodo\"/\"$DAY\"/g" Cargo.toml
cargo test

rm src/data.txt
# Download puzzle input
aoc d -y 2025 -d $DAY_OF_MONTH -I -i ./src/data.txt
# Print puzzle
aoc
#!/bin/bash

set -e
set -u
set -o pipefail

function usage {
	echo "Usage: $0 day01"
}

if [ $# -ne 1 ]; then
	usage
	exit 1
fi

# Regex the day number to make sure it's in the right format
if [[ ! $1 =~ ^day[0-9]{2}$ ]]; then
	echo "Invalid day number"
	usage
	exit 1
fi

# copy the template directory to the new directory
cp -r template "$1"

# use sed to replace dayXX with the day number in the following files:
#   src/main.rs
#   benches/bench.rs
#   Cargo.toml

sed -i "s/dayXX/$1/g" "$1/src/main.rs"
sed -i "s/dayXX/$1/g" "$1/benches/bench.rs"
sed -i "s/dayXX/$1/g" "$1/Cargo.toml"

echo "Created $1"

#!/usr/bin/env bash

set -e

export PATH=$PATH:$(realpath "./libs/bats/bin/")

##
# run localstack once with all the services required in the tests -
# to avoid recreating localstack everytime a different service is being tested.
#
# NOTE: this assumes all services are being tested on the same localstack version.
#
##
../target/debug/sup --stackfile stackfile.json > /dev/null 2>&1

run () {
  for file in "$1"/*
  do
    if [[ -f "$file" && "$file" = "$1/test.bats" ]]; then
      bats "$file"
    fi

    if [[ -d "$file" && "$file" != "$1/libs" ]]; then
      cd "$file"
      run "$file"
      cd ..
    fi
  done
}

run "$PWD"

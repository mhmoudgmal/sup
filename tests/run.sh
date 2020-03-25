#!/usr/bin/env bash

set -e

export PATH=$PATH:$(realpath "./libs/bats/bin/")

run () {
  for file in "$1"/*
  do
    if [[ -f "$file" && "$file" = "$1/test.bats" ]]; then
      bats "$file"
    fi

    if [[ -d "$file" ]]; then
      cd "$file"
      run "$file"
      cd ..
    fi
  done
}

run "$PWD"

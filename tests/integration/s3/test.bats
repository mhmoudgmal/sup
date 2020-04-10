#!/usr/bin/env ../../libs/bats/bin/bats

load '../../libs/bats-support/load'
load '../../libs/bats-assert/load'

@test "s3: creates the bucket and upload the specified files" {
  RUST_LOG=info ../../../target/debug/sup --stackfile stackfile.json 2>&1

  run echo $(
    aws s3 ls --recursive bucket1 --endpoint-url http://localhost:4572
  )

  assert_success
  assert_output --partial "file1"
  assert_output --partial "folder1/file2"
}

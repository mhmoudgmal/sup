#!/usr/bin/env ../../libs/bats/bin/bats

load '../../libs/bats-support/load'
load '../../libs/bats-assert/load'

setup() {
  cd stack
  RUST_LOG=info ../../../../target/debug/lsup --stackfile stackfile.json 2>&1
}

teardown() {
  cd ..
}

@test "lambda: functions exist after stack deployment" {
  local go_fn='test_go_function'
  local node_fn='test_node_function'
  local ruby_fn='test_ruby_function'
  local ruby_fn_with_dep='test_ruby_function_with_dep'

  run echo $(
    aws lambda list-functions --endpoint-url http://localhost:4574 | jq --compact-output '.[] | map(."FunctionName")'
  )

  assert_success
  assert_output --partial $go_fn
  assert_output --partial $node_fn
  assert_output --partial $ruby_fn
  assert_output --partial $ruby_fn_with_dep
}

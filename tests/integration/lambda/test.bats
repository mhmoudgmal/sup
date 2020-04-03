#!/usr/bin/env ../../libs/bats/bin/bats

load '../../libs/bats-support/load'
load '../../libs/bats-assert/load'

setup() {
  cd stack
  if [[ "$BATS_TEST_NUMBER" -eq 1 ]]; then
    RUST_LOG=info ../../../../target/debug/sup --stackfile stackfile.json 2>&1
    # TODO: fix localstack init to wait for all LocalStack services to be ready
    sleep 5
  fi
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
    aws lambda list-functions --endpoint-url http://localhost:4574 \
    | jq --compact-output '.[] | map(."FunctionName")'
  )

  assert_success
  assert_output --partial $go_fn
  assert_output --partial $node_fn
  assert_output --partial $ruby_fn
  assert_output --partial $ruby_fn_with_dep
}

@test "lambda: include all ENV vars from the dot env file and the stack file" {
  run echo $(
    aws lambda list-functions --endpoint-url http://localhost:4574 \
    | jq '."Functions"
    | map({ (."FunctionName"):(."Environment"."Variables") })
    | add
    | with_entries(select(.value."FOO"=="foo" and .value."BAR"=="bar" and .value."BAZ"=="baz"))
    | keys'
  )

  assert_success
  assert_output --partial "test_go_function"
  assert_output --partial "test_node_function"
  assert_output --partial "test_ruby_function"
  assert_output --partial "test_ruby_function_with_dep"
}

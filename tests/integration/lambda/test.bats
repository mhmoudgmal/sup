#!/usr/bin/env ../../libs/bats/bin/bats

load '../../libs/bats-support/load'
load '../../libs/bats-assert/load'

setup() {
  cd stack
  if [[ "$BATS_TEST_NUMBER" -eq 1 ]]; then
    RUST_LOG=info ../../../../target/debug/lsup --stackfile stackfile.json 2>&1
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
  local go_fn_vars='test_go_function={\"BAR\":\"bar\",\"BAZ\":\"baz\",\"FOO\":\"foo\"}'
  local node_fn_vars='test_ruby_function_with_dep={\"BAR\":\"bar\",\"BAZ\":\"baz\",\"FOO\":\"foo\"}'
  local ruby_fn_vars='test_node_function={\"BAR\":\"bar\",\"BAZ\":\"baz\",\"FOO\":\"foo\"}'
  local ruby_fn_with_dep_vars='test_ruby_function={\"BAR\":\"bar\",\"BAZ\":\"baz\",\"FOO\":\"foo\"}'

  run echo $(
    aws lambda list-functions --endpoint-url http://localhost:4574 \
    | jq --compact-output '."Functions" |  map("\(."FunctionName")=\(."Environment"."Variables")")'
  )

  assert_success
  assert_output --partial $go_fn_vars
  assert_output --partial $node_fn_vars
  assert_output --partial $ruby_fn_vars
  assert_output --partial $ruby_fn_with_dep_vars
}

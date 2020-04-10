[![Build Status](https://travis-ci.com/mhmoudgmal/sup.svg?token=r3SpzFYyL6HzDssGTPPR&branch=master)](https://travis-ci.org/mhmoudgmal/sup)

Localstack CLI **(Under development)**

> sup? A simple and easy to use command line interface to automate and simplify working with localstack in the development environments based on AWS services.

> Developer and CI friendly.

### Who is this for?
// TODO

### How does it work?

##### 1. Describe your stack in a `json` or `yaml` file.

```json
// my_stack.json
{
    "localstack_config": {
        "version": "0.10.8",
        "services": ["lambda"],
        "lambda_executer": "docker-reuse",
        "docker_host": "unix:///var/run/docker.sock",
        "recreate": true
    },
    "services": {
        "my_lambda": {
            "runtime": "go1.x",
            "handler": "main",
            "env_file": ".env.test",
            "function_name": "test_go_function",
            "env_vars": {
                "BAR": "bar",
                "BAZ": "baz"
            },
            "files": ["main", ".env.test"],
            "function_path": "."
        }
    }
}
```
**Note** - for more examples of stack descriptions, check [examples](examples)

##### 2. Deploy your stack

```sh
$ RUST_LOG=info sup --stackfile my_stack.json
```

**Note** - if the stack is described in a `json` or `yaml` file with the name `stackfile` then there is no need to specify the `--stackfile` option while deploying the stack.

```sh
$ RUST_LOG=info sup
```

### Supported services

- lambda
- dynamodb
- sns
- sqs
- kinesis
- s3

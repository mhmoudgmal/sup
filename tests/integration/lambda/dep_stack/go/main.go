package main

import (
	"context"
	"fmt"
	"os"

	"github.com/aws/aws-lambda-go/lambda"
)

type Event struct{}

func HandleRequest(ctx context.Context, event Event) (string, error) {
	return fmt.Sprintf("%v", os.Environ()), nil
}

func main() {
	lambda.Start(HandleRequest)
}

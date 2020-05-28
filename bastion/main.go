package main

import (
	"context"
	"flag"
	"log"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	"go.elastic.co/apm/module/apmhttp"
	"google.golang.org/grpc"

	api "github.com/EarvinKayonga/Morning/bastion/api/spec"
)

var (
	echoEndpoint = flag.String("endpoint", "localhost:9090", "Echo Service")
)

func run() error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithInsecure()}
	err := api.RegisterEchoServiceHandlerFromEndpoint(ctx, mux, *echoEndpoint, opts)
	if err != nil {
		return err
	}

	port := ":8080"

	log.Printf("listening on port %s", port)

	return http.ListenAndServe(port, apmhttp.Wrap(mux))
}

func main() {
	flag.Parse()

	if err := run(); err != nil {
		log.Fatal(err)
	}
}

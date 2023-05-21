package main

import (
	"context"
	"fmt"
	"log"

	grpc "google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("127.0.0.1:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to dial: %v", err)
	}
	defer conn.Close()

	client := NewAuthServiceClient(conn)

	authRequest := &AuthRequest{
		Username: "jane",
		Password: "jane",
	}

	authResponse, err := client.Auth(context.Background(), authRequest)
	if err != nil {
		log.Fatalf("Failed to auth: %v", err)
	}

	fmt.Println("Token: ", authResponse.Token)
	fmt.Println("Role: ", authResponse.Role)
}

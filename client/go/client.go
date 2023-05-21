package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"os"

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

	verifyTokenRequest := &VerifyTokenRequest{
		Token: authResponse.Token,
		Role: authResponse.Role,
	}

	verifyTokenResponse, err := client.VerifyToken(context.Background(), verifyTokenRequest)
	if err != nil {
		log.Fatalf("Failed to verify token: %v", err)
	}

	fmt.Println("Token verified: ", verifyTokenResponse.Valid)

	deviceHandler := NewIoTServiceClient(conn)

	for {
		GetDevicesRequest := &GetDevicesRequest{
			Token: verifyTokenRequest,
		}
	
		deviceStream, err := deviceHandler.GetDevices(context.Background(), GetDevicesRequest)
		if err != nil {
			log.Fatalf("Failed to get devices: %v", err)
		}
	
		for {
			device, err := deviceStream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				log.Fatalf("Failed to receive device: %v", err)
				os.Exit(1)
			}
	
			fmt.Println("Received device: ", device)
		}
	}
}

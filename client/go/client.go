package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"time"

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
	getDevicesRequest := &GetDevicesRequest{
		Token: verifyTokenRequest,
	}

	var request_num int
	DeviceRequestLoop:
	for {
		request_num += 1
		fmt.Println("\nRequest number: ", request_num)

		deviceStream, err := deviceHandler.GetDevices(context.Background(), getDevicesRequest)
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
				break DeviceRequestLoop
			}
	
			fmt.Println("Received device: ", device)
		}

		time.Sleep(1 * time.Second)
	}

	fmt.Println("Client finished")
}

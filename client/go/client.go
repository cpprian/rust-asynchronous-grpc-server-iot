package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"math/rand"
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

			sendCommandRequest := &DeviceEvent{
				DeviceId:          device.Id,
				Token:             verifyTokenRequest,
			}
			if device.Type == 0 {
				sendCommandRequest.Value = int32(rand.Intn(100))
			} else if device.Type == 1 {
				sendCommandRequest.TemperatureStep = int32(rand.Intn(10))
				sendCommandRequest.TargetTemperature = int32(rand.Intn(100))
			} else {
				panic("Unknown device type")
			}

			_, err = deviceHandler.SendCommand(context.Background(), sendCommandRequest)
			if err != nil {
				log.Fatalf("Failed to send command: %v", err)
				break DeviceRequestLoop
			}

			fmt.Println("Sent command: ", sendCommandRequest)

			if rand.Intn(10) < 5 {
				fmt.Println("Add access to device: ", device.Id)
				addAccessRequest := &AddAccessRequest{
					Token: verifyTokenRequest,
					DeviceId: device.Id,
				}

				_, err = deviceHandler.AddAccess(context.Background(), addAccessRequest)
				if err != nil {
					log.Fatalf("Failed to add access: %v", err)
					break DeviceRequestLoop
				}
			} else {
				fmt.Println("Remove access to device: ", device.Id)
				removeAccessRequest := &RemoveAccessRequest{
					Token: verifyTokenRequest,
					DeviceId: device.Id,
				}

				_, err = deviceHandler.RemoveAccess(context.Background(), removeAccessRequest)
				if err != nil {
					log.Fatalf("Failed to remove access: %v", err)
					break DeviceRequestLoop
				}
			}
 		}

		time.Sleep(2 * time.Second)
	}

	fmt.Println("Client finished")
}

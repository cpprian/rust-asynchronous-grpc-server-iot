# Project Report: rust-asynchronous-grpc-server-iot

The "rust-asynchronous-grpc-server-iot" project aims to develop a server written in Rust that provides IoT services using gRPC. The project utilizes the Proto3 syntax for defining the gRPC services and messages. It also incorporates JWT authentication for user authorization. Additionally, Python clients are implemented to simulate thermostat and sensor devices, mainly using the RecordStatistics functionality. On the other hand, a client written in Go is developed to utilize the remaining services, with the restriction that only users with Admin privileges can invoke AddAccess and RemoveAccess.

Key Components:
1. Authentication Service:
   - User message: Contains user information including ID, username, password, and role (enum).
   - AuthRequest message: Represents the authentication request containing the username and password.
   - AuthResponse message: Returns the authentication response containing the generated token and user role.
   - VerifyTokenRequest message: Represents the request to verify the token's validity, including the token and expected user role.
   - VerifyTokenResponse message: Returns the result of token verification, indicating whether the token is valid.

2. IoT Device Service:
   - Device message: Represents an IoT device with properties such as ID, name, description, type (enum), and access status.
   - DeviceType enum: Specifies the type of IoT device, either SENSOR or THERMOSTAT.
   - DeviceEvent message: Contains information about a device event, including the device ID and token for verification.
   - GetDevicesRequest message: Represents the request to retrieve devices, including the token for verification.
   - AddAccessRequest message: Represents the request to add access to a device, including the token and device ID.
   - AddAccessResponse message: Returns the result of the add access operation, indicating whether it was successful.
   - RemoveAccessRequest message: Represents the request to remove access from a device, including the token and device ID.
   - RemoveAccessResponse message: Returns the result of the remove access operation, indicating whether it was successful.
   - RecordStatisticsResponse message: Indicates the completion of the record statistics operation.

3. gRPC Services:
   - AuthService: Provides authentication-related services.
     - Auth: Authenticates the user and returns the token and role.
     - VerifyToken: Verifies the token's validity.
   - IoTService: Offers various IoT device-related services.
     - GetDevices: Retrieves a stream of devices based on the provided token.
     - RecordStatistics: Records statistics for a device and returns the completion status.
     - SendCommand: Sends a device event and returns the updated device information.
     - AddAccess: Adds access to a device based on the provided token and device ID.
     - RemoveAccess: Removes access from a device based on the provided token and device ID.

4. Client Implementations:
   - Python Clients: Simulate thermostat and sensor devices and primarily utilize the RecordStatistics functionality.
   - Go Client: Utilizes the remaining services but restricts AddAccess and RemoveAccess to users with Admin privileges.

The project aims to provide a robust and secure IoT server written in Rust, leveraging asynchronous capabilities and gRPC for efficient communication. The integration of JWT authentication ensures secure access to the services, and the client implementations in Python and Go demonstrate the interoperability of the server with different programming languages.

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Python](https://www.python.org/downloads/)
- [Go](https://golang.org/doc/install)
- [Protocol Buffers](https://grpc.io/docs/protoc-installation/)
- [gRPC](https://grpc.io/docs/languages/rust/quickstart/)

## How to run

1. Clone the repository.
2. Navigate to the project directory.
3. Run the following command to build the project:
   ``` cargo run --bin iot-server ```
4. Run the following command to execute the Python clients:
   ``` python3 ./client/python/client.py <device-name> <device-number> ```
    - device-name: Either "thermostat" or "sensor".
    - device-number: The number of devices to simulate (from 1 to 3 for sensor and 1 to 2 for thermostat).
5. Run the following command to execute the Go client:
   1. Navigate to the client directory:
      ``` cd ./client/go/ ```
    2. Run the following command to build the client:
        ``` go run client.go ```
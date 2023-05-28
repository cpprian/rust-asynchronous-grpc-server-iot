import time
import grpc
import sys
import random

import iot_manifest_pb2
import iot_manifest_pb2_grpc

def run(device_type, device_number):
    channel = grpc.insecure_channel('localhost:50051')
    stub = iot_manifest_pb2_grpc.AuthServiceStub(channel)

    auth_request = iot_manifest_pb2.AuthRequest(username=device_type+device_number, password=device_type+device_number)
    auth_response = stub.Auth(auth_request)
    print('Token:', auth_response.token)
    print('Role:', auth_response.role)

    verify_token_request = iot_manifest_pb2.VerifyTokenRequest(token=auth_response.token, role=auth_response.role)
    verify_token_response = stub.VerifyToken(verify_token_request)
    print('Token Valid:', verify_token_response.valid)

    iot_service = iot_manifest_pb2_grpc.IoTServiceStub(channel)
    data = create_default_device_data(device_type, device_number)
    while True:
        device_data = process_device(data, device_type)
        print('Slow business logic...')
        time.sleep(3)

        iot_service.RecordStatistics(device_data)

def process_device(data, device_type):
    if device_type == "thermostat":
        return process_thermostat(data)
    elif device_type == "sensor":
        return process_sensor(data)
    
def process_thermostat(data):
    if random.random() < 0.2 and data.temperature > data.minTemperature:
        data.temperature -= data.temperatureStep
        print('Decreasing temperature to', data.temperature)
    elif random.random() > 0.6 and data.temperature < data.maxTemperature:
        data.temperature += data.temperatureStep
        print('Increasing temperature to', data.temperature)
    
    print('Nothing to do with temperature', data.temperature)
    return data

def process_sensor(data):
    data.value = random.randint(data.min, data.max)
    print('Setting sensor value to', data.value)
    return data

def create_default_device_data(device_type, device_number):
    data = iot_manifest_pb2.Device()
    data.id = int(device_number)
    data.name = ""
    data.description = ""
    data.hasAccess = True

    if device_type == "thermostat":
        data.type = iot_manifest_pb2.THERMOSTAT
        data.temperature = 0
        data.targetTemperature = 0
        data.temperatureStep = 1
        data.minTemperature = 0
        data.maxTemperature = 100
    elif device_type == "sensor":
        data.type = iot_manifest_pb2.SENSOR
        data.value = 0
        data.min = 0
        data.max = 100

    return data

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage: python client.py <device_type> <device_number>')
        sys.exit(1)

    device_type = sys.argv[1]
    device_number = sys.argv[2]
    print('Device Type:', device_type)
    print('Device Number:', device_number)
    run(device_type, device_number)

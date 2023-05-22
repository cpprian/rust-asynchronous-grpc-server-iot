import grpc

import iot_manifest_pb2
import iot_manifest_pb2_grpc

def run():
    channel = grpc.insecure_channel('localhost:50051')
    stub = iot_manifest_pb2_grpc.AuthServiceStub(channel)

    auth_request = iot_manifest_pb2.AuthRequest(username='sensor01', password='sensor01')
    auth_response = stub.Auth(auth_request)
    print('Token:', auth_response.token)
    print('Role:', auth_response.role)

    verify_token_request = iot_manifest_pb2.VerifyTokenRequest(token=auth_response.token, role=auth_response.role)
    verify_token_response = stub.VerifyToken(verify_token_request)
    print('Token Valid:', verify_token_response.valid)

    iot_service = iot_manifest_pb2_grpc.IoTServiceStub(channel)
    while True:
        iot_service.RecordStatistics()


if __name__ == '__main__':
    run()

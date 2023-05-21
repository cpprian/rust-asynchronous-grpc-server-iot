# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: iot_manifest.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x12iot_manifest.proto\x12\x0ciot_manifest\"X\n\x04User\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x10\n\x08username\x18\x02 \x01(\t\x12\x10\n\x08password\x18\x03 \x01(\t\x12 \n\x04role\x18\x04 \x01(\x0e\x32\x12.iot_manifest.Role\"1\n\x0b\x41uthRequest\x12\x10\n\x08username\x18\x01 \x01(\t\x12\x10\n\x08password\x18\x02 \x01(\t\"?\n\x0c\x41uthResponse\x12\r\n\x05token\x18\x01 \x01(\t\x12 \n\x04role\x18\x02 \x01(\x0e\x32\x12.iot_manifest.Role\"E\n\x12VerifyTokenRequest\x12\r\n\x05token\x18\x01 \x01(\t\x12 \n\x04role\x18\x02 \x01(\x0e\x32\x12.iot_manifest.Role\"$\n\x13VerifyTokenResponse\x12\r\n\x05valid\x18\x01 \x01(\x08\"\x94\x02\n\x06\x44\x65vice\x12\n\n\x02id\x18\x01 \x01(\x05\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12&\n\x04type\x18\x04 \x01(\x0e\x32\x18.iot_manifest.DeviceType\x12\x11\n\thasAccess\x18\x05 \x01(\x08\x12\x13\n\x0btemperature\x18\x06 \x01(\x05\x12\x19\n\x11targetTemperature\x18\x07 \x01(\x05\x12\x17\n\x0ftemperatureStep\x18\x08 \x01(\x05\x12\x16\n\x0eminTemperature\x18\t \x01(\x05\x12\x16\n\x0emaxTemperature\x18\n \x01(\x05\x12\r\n\x05value\x18\x0b \x01(\x05\x12\x0b\n\x03min\x18\x0c \x01(\x05\x12\x0b\n\x03max\x18\r \x01(\x05\"~\n\x0b\x44\x65viceEvent\x12\x10\n\x08\x64\x65viceId\x18\x01 \x01(\x05\x12\x1a\n\x12\x63ommandDescription\x18\x02 \x01(\t\x12\x19\n\x11targetTemperature\x18\x03 \x01(\x05\x12\x17\n\x0ftemperatureStep\x18\x04 \x01(\x05\x12\r\n\x05value\x18\x05 \x01(\x05\"D\n\x11GetDevicesRequest\x12/\n\x05token\x18\x01 \x01(\x0b\x32 .iot_manifest.VerifyTokenRequest\"V\n\x10\x41\x64\x64\x41\x63\x63\x65ssRequest\x12/\n\x05token\x18\x01 \x01(\x0b\x32 .iot_manifest.VerifyTokenRequest\x12\x11\n\tdeviceIds\x18\x02 \x03(\x05\"\\\n\x11\x41\x64\x64\x41\x63\x63\x65ssResponse\x12%\n\x07\x64\x65vices\x18\x01 \x01(\x0b\x32\x14.iot_manifest.Device\x12\x0f\n\x07success\x18\x02 \x01(\x08\x12\x0f\n\x07message\x18\x03 \x01(\t\"X\n\x13RemoveAccessRequest\x12/\n\x05token\x18\x01 \x01(\x0b\x32 .iot_manifest.VerifyTokenRequest\x12\x10\n\x08\x64\x65viceId\x18\x02 \x01(\x05\"_\n\x14RemoveAccessResponse\x12%\n\x07\x64\x65vices\x18\x01 \x03(\x0b\x32\x14.iot_manifest.Device\x12\x0f\n\x07success\x18\x02 \x01(\x08\x12\x0f\n\x07message\x18\x03 \x01(\t\"\x1a\n\x18RecordStatisticsResponse*\x1b\n\x04Role\x12\t\n\x05\x41\x44MIN\x10\x00\x12\x08\n\x04USER\x10\x01*(\n\nDeviceType\x12\n\n\x06SENSOR\x10\x00\x12\x0e\n\nTHERMOSTAT\x10\x01\x32\xa0\x01\n\x0b\x41uthService\x12=\n\x04\x41uth\x12\x19.iot_manifest.AuthRequest\x1a\x1a.iot_manifest.AuthResponse\x12R\n\x0bVerifyToken\x12 .iot_manifest.VerifyTokenRequest\x1a!.iot_manifest.VerifyTokenResponse2\xa2\x03\n\nIoTService\x12G\n\nGetDevices\x12\x1f.iot_manifest.GetDevicesRequest\x1a\x14.iot_manifest.Device\"\x00\x30\x01\x12T\n\x10RecordStatistics\x12\x14.iot_manifest.Device\x1a&.iot_manifest.RecordStatisticsResponse\"\x00(\x01\x12\x44\n\x0bSendCommand\x12\x19.iot_manifest.DeviceEvent\x1a\x14.iot_manifest.Device\"\x00(\x01\x30\x01\x12R\n\tAddAccess\x12\x1e.iot_manifest.AddAccessRequest\x1a\x1f.iot_manifest.AddAccessResponse\"\x00(\x01\x30\x01\x12[\n\x0cRemoveAccess\x12!.iot_manifest.RemoveAccessRequest\x1a\".iot_manifest.RemoveAccessResponse\"\x00(\x01\x30\x01\x42\x03Z\x01.b\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'iot_manifest_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  DESCRIPTOR._serialized_options = b'Z\001.'
  _ROLE._serialized_start=1225
  _ROLE._serialized_end=1252
  _DEVICETYPE._serialized_start=1254
  _DEVICETYPE._serialized_end=1294
  _USER._serialized_start=36
  _USER._serialized_end=124
  _AUTHREQUEST._serialized_start=126
  _AUTHREQUEST._serialized_end=175
  _AUTHRESPONSE._serialized_start=177
  _AUTHRESPONSE._serialized_end=240
  _VERIFYTOKENREQUEST._serialized_start=242
  _VERIFYTOKENREQUEST._serialized_end=311
  _VERIFYTOKENRESPONSE._serialized_start=313
  _VERIFYTOKENRESPONSE._serialized_end=349
  _DEVICE._serialized_start=352
  _DEVICE._serialized_end=628
  _DEVICEEVENT._serialized_start=630
  _DEVICEEVENT._serialized_end=756
  _GETDEVICESREQUEST._serialized_start=758
  _GETDEVICESREQUEST._serialized_end=826
  _ADDACCESSREQUEST._serialized_start=828
  _ADDACCESSREQUEST._serialized_end=914
  _ADDACCESSRESPONSE._serialized_start=916
  _ADDACCESSRESPONSE._serialized_end=1008
  _REMOVEACCESSREQUEST._serialized_start=1010
  _REMOVEACCESSREQUEST._serialized_end=1098
  _REMOVEACCESSRESPONSE._serialized_start=1100
  _REMOVEACCESSRESPONSE._serialized_end=1195
  _RECORDSTATISTICSRESPONSE._serialized_start=1197
  _RECORDSTATISTICSRESPONSE._serialized_end=1223
  _AUTHSERVICE._serialized_start=1297
  _AUTHSERVICE._serialized_end=1457
  _IOTSERVICE._serialized_start=1460
  _IOTSERVICE._serialized_end=1878
# @@protoc_insertion_point(module_scope)

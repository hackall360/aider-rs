// This is a generated file - do not edit.
//
// Generated from proto/aider.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use openRequestDescriptor instead')
const OpenRequest$json = {
  '1': 'OpenRequest',
  '2': [
    {'1': 'client_version', '3': 1, '4': 1, '5': 9, '10': 'clientVersion'},
  ],
};

/// Descriptor for `OpenRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openRequestDescriptor = $convert.base64Decode(
    'CgtPcGVuUmVxdWVzdBIlCg5jbGllbnRfdmVyc2lvbhgBIAEoCVINY2xpZW50VmVyc2lvbg==');

@$core.Deprecated('Use openResponseDescriptor instead')
const OpenResponse$json = {
  '1': 'OpenResponse',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'server_version', '3': 2, '4': 1, '5': 9, '10': 'serverVersion'},
  ],
};

/// Descriptor for `OpenResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openResponseDescriptor = $convert.base64Decode(
    'CgxPcGVuUmVzcG9uc2USHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklkEiUKDnNlcnZlcl'
    '92ZXJzaW9uGAIgASgJUg1zZXJ2ZXJWZXJzaW9u');

@$core.Deprecated('Use closeRequestDescriptor instead')
const CloseRequest$json = {
  '1': 'CloseRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `CloseRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List closeRequestDescriptor = $convert.base64Decode(
    'CgxDbG9zZVJlcXVlc3QSHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklk');

@$core.Deprecated('Use closeResponseDescriptor instead')
const CloseResponse$json = {
  '1': 'CloseResponse',
};

/// Descriptor for `CloseResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List closeResponseDescriptor =
    $convert.base64Decode('Cg1DbG9zZVJlc3BvbnNl');

@$core.Deprecated('Use sendMessageRequestDescriptor instead')
const SendMessageRequest$json = {
  '1': 'SendMessageRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `SendMessageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendMessageRequestDescriptor = $convert.base64Decode(
    'ChJTZW5kTWVzc2FnZVJlcXVlc3QSHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklkEhgKB2'
    '1lc3NhZ2UYAiABKAlSB21lc3NhZ2U=');

@$core.Deprecated('Use setFilesRequestDescriptor instead')
const SetFilesRequest$json = {
  '1': 'SetFilesRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {
      '1': 'files',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.aider.v1.SetFilesRequest.File',
      '10': 'files'
    },
  ],
  '3': [SetFilesRequest_File$json],
};

@$core.Deprecated('Use setFilesRequestDescriptor instead')
const SetFilesRequest_File$json = {
  '1': 'File',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'content', '3': 2, '4': 1, '5': 9, '10': 'content'},
  ],
};

/// Descriptor for `SetFilesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setFilesRequestDescriptor = $convert.base64Decode(
    'Cg9TZXRGaWxlc1JlcXVlc3QSHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklkEjQKBWZpbG'
    'VzGAIgAygLMh4uYWlkZXIudjEuU2V0RmlsZXNSZXF1ZXN0LkZpbGVSBWZpbGVzGjQKBEZpbGUS'
    'EgoEcGF0aBgBIAEoCVIEcGF0aBIYCgdjb250ZW50GAIgASgJUgdjb250ZW50');

@$core.Deprecated('Use emptyDescriptor instead')
const Empty$json = {
  '1': 'Empty',
};

/// Descriptor for `Empty`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emptyDescriptor =
    $convert.base64Decode('CgVFbXB0eQ==');

@$core.Deprecated('Use tokenChunkDescriptor instead')
const TokenChunk$json = {
  '1': 'TokenChunk',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
    {'1': 'done', '3': 2, '4': 1, '5': 8, '10': 'done'},
  ],
};

/// Descriptor for `TokenChunk`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List tokenChunkDescriptor = $convert.base64Decode(
    'CgpUb2tlbkNodW5rEhIKBHRleHQYASABKAlSBHRleHQSEgoEZG9uZRgCIAEoCFIEZG9uZQ==');

@$core.Deprecated('Use getMapRequestDescriptor instead')
const GetMapRequest$json = {
  '1': 'GetMapRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `GetMapRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getMapRequestDescriptor = $convert.base64Decode(
    'Cg1HZXRNYXBSZXF1ZXN0Eh0KCnNlc3Npb25faWQYASABKAlSCXNlc3Npb25JZA==');

@$core.Deprecated('Use getMapResponseDescriptor instead')
const GetMapResponse$json = {
  '1': 'GetMapResponse',
  '2': [
    {'1': 'map_json', '3': 1, '4': 1, '5': 9, '10': 'mapJson'},
  ],
};

/// Descriptor for `GetMapResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getMapResponseDescriptor = $convert.base64Decode(
    'Cg5HZXRNYXBSZXNwb25zZRIZCghtYXBfanNvbhgBIAEoCVIHbWFwSnNvbg==');

@$core.Deprecated('Use previewRequestDescriptor instead')
const PreviewRequest$json = {
  '1': 'PreviewRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'diff', '3': 2, '4': 1, '5': 9, '10': 'diff'},
  ],
};

/// Descriptor for `PreviewRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List previewRequestDescriptor = $convert.base64Decode(
    'Cg5QcmV2aWV3UmVxdWVzdBIdCgpzZXNzaW9uX2lkGAEgASgJUglzZXNzaW9uSWQSEgoEZGlmZh'
    'gCIAEoCVIEZGlmZg==');

@$core.Deprecated('Use previewResponseDescriptor instead')
const PreviewResponse$json = {
  '1': 'PreviewResponse',
  '2': [
    {'1': 'preview', '3': 1, '4': 1, '5': 9, '10': 'preview'},
  ],
};

/// Descriptor for `PreviewResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List previewResponseDescriptor = $convert.base64Decode(
    'Cg9QcmV2aWV3UmVzcG9uc2USGAoHcHJldmlldxgBIAEoCVIHcHJldmlldw==');

@$core.Deprecated('Use applyRequestDescriptor instead')
const ApplyRequest$json = {
  '1': 'ApplyRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'diff', '3': 2, '4': 1, '5': 9, '10': 'diff'},
  ],
};

/// Descriptor for `ApplyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applyRequestDescriptor = $convert.base64Decode(
    'CgxBcHBseVJlcXVlc3QSHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklkEhIKBGRpZmYYAi'
    'ABKAlSBGRpZmY=');

@$core.Deprecated('Use applyResponseDescriptor instead')
const ApplyResponse$json = {
  '1': 'ApplyResponse',
};

/// Descriptor for `ApplyResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List applyResponseDescriptor =
    $convert.base64Decode('Cg1BcHBseVJlc3BvbnNl');

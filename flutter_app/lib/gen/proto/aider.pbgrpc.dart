// This is a generated file - do not edit.
//
// Generated from proto/aider.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'aider.pb.dart' as $0;

export 'aider.pb.dart';

@$pb.GrpcServiceName('aider.v1.SessionService')
class SessionServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SessionServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.OpenResponse> open(
    $0.OpenRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$open, request, options: options);
  }

  $grpc.ResponseFuture<$0.CloseResponse> close(
    $0.CloseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$close, request, options: options);
  }

  $grpc.ResponseStream<$0.TokenChunk> sendMessage(
    $0.SendMessageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(
        _$sendMessage, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.Empty> setFiles(
    $0.SetFilesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$setFiles, request, options: options);
  }

  // method descriptors

  static final _$open = $grpc.ClientMethod<$0.OpenRequest, $0.OpenResponse>(
      '/aider.v1.SessionService/Open',
      ($0.OpenRequest value) => value.writeToBuffer(),
      $0.OpenResponse.fromBuffer);
  static final _$close = $grpc.ClientMethod<$0.CloseRequest, $0.CloseResponse>(
      '/aider.v1.SessionService/Close',
      ($0.CloseRequest value) => value.writeToBuffer(),
      $0.CloseResponse.fromBuffer);
  static final _$sendMessage =
      $grpc.ClientMethod<$0.SendMessageRequest, $0.TokenChunk>(
          '/aider.v1.SessionService/SendMessage',
          ($0.SendMessageRequest value) => value.writeToBuffer(),
          $0.TokenChunk.fromBuffer);
  static final _$setFiles = $grpc.ClientMethod<$0.SetFilesRequest, $0.Empty>(
      '/aider.v1.SessionService/SetFiles',
      ($0.SetFilesRequest value) => value.writeToBuffer(),
      $0.Empty.fromBuffer);
}

@$pb.GrpcServiceName('aider.v1.SessionService')
abstract class SessionServiceBase extends $grpc.Service {
  $core.String get $name => 'aider.v1.SessionService';

  SessionServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.OpenRequest, $0.OpenResponse>(
        'Open',
        open_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.OpenRequest.fromBuffer(value),
        ($0.OpenResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CloseRequest, $0.CloseResponse>(
        'Close',
        close_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CloseRequest.fromBuffer(value),
        ($0.CloseResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SendMessageRequest, $0.TokenChunk>(
        'SendMessage',
        sendMessage_Pre,
        false,
        true,
        ($core.List<$core.int> value) =>
            $0.SendMessageRequest.fromBuffer(value),
        ($0.TokenChunk value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SetFilesRequest, $0.Empty>(
        'SetFiles',
        setFiles_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SetFilesRequest.fromBuffer(value),
        ($0.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.OpenResponse> open_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.OpenRequest> $request) async {
    return open($call, await $request);
  }

  $async.Future<$0.OpenResponse> open(
      $grpc.ServiceCall call, $0.OpenRequest request);

  $async.Future<$0.CloseResponse> close_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.CloseRequest> $request) async {
    return close($call, await $request);
  }

  $async.Future<$0.CloseResponse> close(
      $grpc.ServiceCall call, $0.CloseRequest request);

  $async.Stream<$0.TokenChunk> sendMessage_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SendMessageRequest> $request) async* {
    yield* sendMessage($call, await $request);
  }

  $async.Stream<$0.TokenChunk> sendMessage(
      $grpc.ServiceCall call, $0.SendMessageRequest request);

  $async.Future<$0.Empty> setFiles_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SetFilesRequest> $request) async {
    return setFiles($call, await $request);
  }

  $async.Future<$0.Empty> setFiles(
      $grpc.ServiceCall call, $0.SetFilesRequest request);
}

@$pb.GrpcServiceName('aider.v1.RepoMapService')
class RepoMapServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  RepoMapServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.GetMapResponse> getMap(
    $0.GetMapRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getMap, request, options: options);
  }

  $grpc.ResponseFuture<$0.SnippetResponse> getSnippet(
    $0.SnippetRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getSnippet, request, options: options);
  }

  // method descriptors

  static final _$getMap =
      $grpc.ClientMethod<$0.GetMapRequest, $0.GetMapResponse>(
          '/aider.v1.RepoMapService/GetMap',
          ($0.GetMapRequest value) => value.writeToBuffer(),
          $0.GetMapResponse.fromBuffer);
  static final _$getSnippet =
      $grpc.ClientMethod<$0.SnippetRequest, $0.SnippetResponse>(
          '/aider.v1.RepoMapService/GetSnippet',
          ($0.SnippetRequest value) => value.writeToBuffer(),
          $0.SnippetResponse.fromBuffer);
}

@$pb.GrpcServiceName('aider.v1.RepoMapService')
abstract class RepoMapServiceBase extends $grpc.Service {
  $core.String get $name => 'aider.v1.RepoMapService';

  RepoMapServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetMapRequest, $0.GetMapResponse>(
        'GetMap',
        getMap_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetMapRequest.fromBuffer(value),
        ($0.GetMapResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SnippetRequest, $0.SnippetResponse>(
        'GetSnippet',
        getSnippet_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SnippetRequest.fromBuffer(value),
        ($0.SnippetResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.GetMapResponse> getMap_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.GetMapRequest> $request) async {
    return getMap($call, await $request);
  }

  $async.Future<$0.SnippetResponse> getSnippet_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.SnippetRequest> $request) async {
    return getSnippet($call, await $request);
  }

  $async.Future<$0.GetMapResponse> getMap(
      $grpc.ServiceCall call, $0.GetMapRequest request);

  $async.Future<$0.SnippetResponse> getSnippet(
      $grpc.ServiceCall call, $0.SnippetRequest request);
}

@$pb.GrpcServiceName('aider.v1.DiffService')
class DiffServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  DiffServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.PreviewResponse> preview(
    $0.PreviewRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$preview, request, options: options);
  }

  $grpc.ResponseFuture<$0.ApplyResponse> apply(
    $0.ApplyRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$apply, request, options: options);
  }

  // method descriptors

  static final _$preview =
      $grpc.ClientMethod<$0.PreviewRequest, $0.PreviewResponse>(
          '/aider.v1.DiffService/Preview',
          ($0.PreviewRequest value) => value.writeToBuffer(),
          $0.PreviewResponse.fromBuffer);
  static final _$apply = $grpc.ClientMethod<$0.ApplyRequest, $0.ApplyResponse>(
      '/aider.v1.DiffService/Apply',
      ($0.ApplyRequest value) => value.writeToBuffer(),
      $0.ApplyResponse.fromBuffer);
}

@$pb.GrpcServiceName('aider.v1.DiffService')
abstract class DiffServiceBase extends $grpc.Service {
  $core.String get $name => 'aider.v1.DiffService';

  DiffServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.PreviewRequest, $0.PreviewResponse>(
        'Preview',
        preview_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.PreviewRequest.fromBuffer(value),
        ($0.PreviewResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ApplyRequest, $0.ApplyResponse>(
        'Apply',
        apply_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ApplyRequest.fromBuffer(value),
        ($0.ApplyResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.PreviewResponse> preview_Pre($grpc.ServiceCall $call,
      $async.Future<$0.PreviewRequest> $request) async {
    return preview($call, await $request);
  }

  $async.Future<$0.PreviewResponse> preview(
      $grpc.ServiceCall call, $0.PreviewRequest request);

  $async.Future<$0.ApplyResponse> apply_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.ApplyRequest> $request) async {
    return apply($call, await $request);
  }

  $async.Future<$0.ApplyResponse> apply(
      $grpc.ServiceCall call, $0.ApplyRequest request);
}

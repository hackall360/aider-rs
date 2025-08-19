// This is a generated file - do not edit.
//
// Generated from proto/aider.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class OpenRequest extends $pb.GeneratedMessage {
  factory OpenRequest({
    $core.String? clientVersion,
  }) {
    final result = create();
    if (clientVersion != null) result.clientVersion = clientVersion;
    return result;
  }

  OpenRequest._();

  factory OpenRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory OpenRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OpenRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'clientVersion')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenRequest clone() => OpenRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenRequest copyWith(void Function(OpenRequest) updates) =>
      super.copyWith((message) => updates(message as OpenRequest))
          as OpenRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OpenRequest create() => OpenRequest._();
  @$core.override
  OpenRequest createEmptyInstance() => create();
  static $pb.PbList<OpenRequest> createRepeated() => $pb.PbList<OpenRequest>();
  @$core.pragma('dart2js:noInline')
  static OpenRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OpenRequest>(create);
  static OpenRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get clientVersion => $_getSZ(0);
  @$pb.TagNumber(1)
  set clientVersion($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasClientVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearClientVersion() => $_clearField(1);
}

class OpenResponse extends $pb.GeneratedMessage {
  factory OpenResponse({
    $core.String? sessionId,
    $core.String? serverVersion,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (serverVersion != null) result.serverVersion = serverVersion;
    return result;
  }

  OpenResponse._();

  factory OpenResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory OpenResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OpenResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'serverVersion')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenResponse clone() => OpenResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenResponse copyWith(void Function(OpenResponse) updates) =>
      super.copyWith((message) => updates(message as OpenResponse))
          as OpenResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OpenResponse create() => OpenResponse._();
  @$core.override
  OpenResponse createEmptyInstance() => create();
  static $pb.PbList<OpenResponse> createRepeated() =>
      $pb.PbList<OpenResponse>();
  @$core.pragma('dart2js:noInline')
  static OpenResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<OpenResponse>(create);
  static OpenResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get serverVersion => $_getSZ(1);
  @$pb.TagNumber(2)
  set serverVersion($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasServerVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearServerVersion() => $_clearField(2);
}

class CloseRequest extends $pb.GeneratedMessage {
  factory CloseRequest({
    $core.String? sessionId,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    return result;
  }

  CloseRequest._();

  factory CloseRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CloseRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CloseRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CloseRequest clone() => CloseRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CloseRequest copyWith(void Function(CloseRequest) updates) =>
      super.copyWith((message) => updates(message as CloseRequest))
          as CloseRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CloseRequest create() => CloseRequest._();
  @$core.override
  CloseRequest createEmptyInstance() => create();
  static $pb.PbList<CloseRequest> createRepeated() =>
      $pb.PbList<CloseRequest>();
  @$core.pragma('dart2js:noInline')
  static CloseRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CloseRequest>(create);
  static CloseRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);
}

class CloseResponse extends $pb.GeneratedMessage {
  factory CloseResponse() => create();

  CloseResponse._();

  factory CloseResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CloseResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CloseResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CloseResponse clone() => CloseResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CloseResponse copyWith(void Function(CloseResponse) updates) =>
      super.copyWith((message) => updates(message as CloseResponse))
          as CloseResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CloseResponse create() => CloseResponse._();
  @$core.override
  CloseResponse createEmptyInstance() => create();
  static $pb.PbList<CloseResponse> createRepeated() =>
      $pb.PbList<CloseResponse>();
  @$core.pragma('dart2js:noInline')
  static CloseResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CloseResponse>(create);
  static CloseResponse? _defaultInstance;
}

class SendMessageRequest extends $pb.GeneratedMessage {
  factory SendMessageRequest({
    $core.String? sessionId,
    $core.String? message,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (message != null) result.message = message;
    return result;
  }

  SendMessageRequest._();

  factory SendMessageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SendMessageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SendMessageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessageRequest clone() => SendMessageRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SendMessageRequest copyWith(void Function(SendMessageRequest) updates) =>
      super.copyWith((message) => updates(message as SendMessageRequest))
          as SendMessageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SendMessageRequest create() => SendMessageRequest._();
  @$core.override
  SendMessageRequest createEmptyInstance() => create();
  static $pb.PbList<SendMessageRequest> createRepeated() =>
      $pb.PbList<SendMessageRequest>();
  @$core.pragma('dart2js:noInline')
  static SendMessageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SendMessageRequest>(create);
  static SendMessageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

class SetFilesRequest_File extends $pb.GeneratedMessage {
  factory SetFilesRequest_File({
    $core.String? path,
    $core.String? content,
  }) {
    final result = create();
    if (path != null) result.path = path;
    if (content != null) result.content = content;
    return result;
  }

  SetFilesRequest_File._();

  factory SetFilesRequest_File.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SetFilesRequest_File.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetFilesRequest.File',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetFilesRequest_File clone() =>
      SetFilesRequest_File()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetFilesRequest_File copyWith(void Function(SetFilesRequest_File) updates) =>
      super.copyWith((message) => updates(message as SetFilesRequest_File))
          as SetFilesRequest_File;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetFilesRequest_File create() => SetFilesRequest_File._();
  @$core.override
  SetFilesRequest_File createEmptyInstance() => create();
  static $pb.PbList<SetFilesRequest_File> createRepeated() =>
      $pb.PbList<SetFilesRequest_File>();
  @$core.pragma('dart2js:noInline')
  static SetFilesRequest_File getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetFilesRequest_File>(create);
  static SetFilesRequest_File? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);
}

class SetFilesRequest extends $pb.GeneratedMessage {
  factory SetFilesRequest({
    $core.String? sessionId,
    $core.Iterable<SetFilesRequest_File>? files,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (files != null) result.files.addAll(files);
    return result;
  }

  SetFilesRequest._();

  factory SetFilesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SetFilesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SetFilesRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..pc<SetFilesRequest_File>(
        2, _omitFieldNames ? '' : 'files', $pb.PbFieldType.PM,
        subBuilder: SetFilesRequest_File.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetFilesRequest clone() => SetFilesRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SetFilesRequest copyWith(void Function(SetFilesRequest) updates) =>
      super.copyWith((message) => updates(message as SetFilesRequest))
          as SetFilesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetFilesRequest create() => SetFilesRequest._();
  @$core.override
  SetFilesRequest createEmptyInstance() => create();
  static $pb.PbList<SetFilesRequest> createRepeated() =>
      $pb.PbList<SetFilesRequest>();
  @$core.pragma('dart2js:noInline')
  static SetFilesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SetFilesRequest>(create);
  static SetFilesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<SetFilesRequest_File> get files => $_getList(1);
}

class Empty extends $pb.GeneratedMessage {
  factory Empty() => create();

  Empty._();

  factory Empty.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Empty.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Empty',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Empty clone() => Empty()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Empty copyWith(void Function(Empty) updates) =>
      super.copyWith((message) => updates(message as Empty)) as Empty;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Empty create() => Empty._();
  @$core.override
  Empty createEmptyInstance() => create();
  static $pb.PbList<Empty> createRepeated() => $pb.PbList<Empty>();
  @$core.pragma('dart2js:noInline')
  static Empty getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Empty>(create);
  static Empty? _defaultInstance;
}

class TokenChunk extends $pb.GeneratedMessage {
  factory TokenChunk({
    $core.String? text,
    $core.bool? done,
  }) {
    final result = create();
    if (text != null) result.text = text;
    if (done != null) result.done = done;
    return result;
  }

  TokenChunk._();

  factory TokenChunk.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory TokenChunk.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TokenChunk',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..aOB(2, _omitFieldNames ? '' : 'done')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TokenChunk clone() => TokenChunk()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  TokenChunk copyWith(void Function(TokenChunk) updates) =>
      super.copyWith((message) => updates(message as TokenChunk)) as TokenChunk;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TokenChunk create() => TokenChunk._();
  @$core.override
  TokenChunk createEmptyInstance() => create();
  static $pb.PbList<TokenChunk> createRepeated() => $pb.PbList<TokenChunk>();
  @$core.pragma('dart2js:noInline')
  static TokenChunk getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TokenChunk>(create);
  static TokenChunk? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get done => $_getBF(1);
  @$pb.TagNumber(2)
  set done($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDone() => $_has(1);
  @$pb.TagNumber(2)
  void clearDone() => $_clearField(2);
}

class GetMapRequest extends $pb.GeneratedMessage {
  factory GetMapRequest({
    $core.String? sessionId,
    $core.int? tokenBudget,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (tokenBudget != null) result.tokenBudget = tokenBudget;
    return result;
  }

  GetMapRequest._();

  factory GetMapRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetMapRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetMapRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'tokenBudget', $pb.PbFieldType.O3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMapRequest clone() => GetMapRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMapRequest copyWith(void Function(GetMapRequest) updates) =>
      super.copyWith((message) => updates(message as GetMapRequest))
          as GetMapRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetMapRequest create() => GetMapRequest._();
  @$core.override
  GetMapRequest createEmptyInstance() => create();
  static $pb.PbList<GetMapRequest> createRepeated() =>
      $pb.PbList<GetMapRequest>();
  @$core.pragma('dart2js:noInline')
  static GetMapRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetMapRequest>(create);
  static GetMapRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get tokenBudget => $_getIZ(1);
  @$pb.TagNumber(2)
  set tokenBudget($core.int value) => $_setSignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasTokenBudget() => $_has(1);
  @$pb.TagNumber(2)
  void clearTokenBudget() => $_clearField(2);
}

class SnippetRequest extends $pb.GeneratedMessage {
  factory SnippetRequest({
    $core.String? sessionId,
    $core.String? path,
    $core.int? line,
    $core.int? context,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (path != null) result.path = path;
    if (line != null) result.line = line;
    if (context != null) result.context = context;
    return result;
  }

  SnippetRequest._();

  factory SnippetRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SnippetRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SnippetRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'path')
    ..a<$core.int>(3, _omitFieldNames ? '' : 'line', $pb.PbFieldType.O3)
    ..a<$core.int>(4, _omitFieldNames ? '' : 'context', $pb.PbFieldType.O3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SnippetRequest clone() => SnippetRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SnippetRequest copyWith(void Function(SnippetRequest) updates) =>
      super.copyWith((message) => updates(message as SnippetRequest))
          as SnippetRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SnippetRequest create() => SnippetRequest._();
  @$core.override
  SnippetRequest createEmptyInstance() => create();
  static $pb.PbList<SnippetRequest> createRepeated() =>
      $pb.PbList<SnippetRequest>();
  @$core.pragma('dart2js:noInline')
  static SnippetRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SnippetRequest>(create);
  static SnippetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get line => $_getIZ(2);
  @$pb.TagNumber(3)
  set line($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLine() => $_has(2);
  @$pb.TagNumber(3)
  void clearLine() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get context => $_getIZ(3);
  @$pb.TagNumber(4)
  set context($core.int value) => $_setSignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasContext() => $_has(3);
  @$pb.TagNumber(4)
  void clearContext() => $_clearField(4);
}

class SnippetResponse extends $pb.GeneratedMessage {
  factory SnippetResponse({
    $core.String? content,
  }) {
    final result = create();
    if (content != null) result.content = content;
    return result;
  }

  SnippetResponse._();

  factory SnippetResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SnippetResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SnippetResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SnippetResponse clone() => SnippetResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SnippetResponse copyWith(void Function(SnippetResponse) updates) =>
      super.copyWith((message) => updates(message as SnippetResponse))
          as SnippetResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SnippetResponse create() => SnippetResponse._();
  @$core.override
  SnippetResponse createEmptyInstance() => create();
  static $pb.PbList<SnippetResponse> createRepeated() =>
      $pb.PbList<SnippetResponse>();
  @$core.pragma('dart2js:noInline')
  static SnippetResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SnippetResponse>(create);
  static SnippetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);
}

class GetMapResponse extends $pb.GeneratedMessage {
  factory GetMapResponse({
    $core.String? mapJson,
  }) {
    final result = create();
    if (mapJson != null) result.mapJson = mapJson;
    return result;
  }

  GetMapResponse._();

  factory GetMapResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetMapResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetMapResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'mapJson')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMapResponse clone() => GetMapResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMapResponse copyWith(void Function(GetMapResponse) updates) =>
      super.copyWith((message) => updates(message as GetMapResponse))
          as GetMapResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetMapResponse create() => GetMapResponse._();
  @$core.override
  GetMapResponse createEmptyInstance() => create();
  static $pb.PbList<GetMapResponse> createRepeated() =>
      $pb.PbList<GetMapResponse>();
  @$core.pragma('dart2js:noInline')
  static GetMapResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetMapResponse>(create);
  static GetMapResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get mapJson => $_getSZ(0);
  @$pb.TagNumber(1)
  set mapJson($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMapJson() => $_has(0);
  @$pb.TagNumber(1)
  void clearMapJson() => $_clearField(1);
}

class PreviewRequest extends $pb.GeneratedMessage {
  factory PreviewRequest({
    $core.String? sessionId,
    $core.String? diff,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (diff != null) result.diff = diff;
    return result;
  }

  PreviewRequest._();

  factory PreviewRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PreviewRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PreviewRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'diff')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewRequest clone() => PreviewRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewRequest copyWith(void Function(PreviewRequest) updates) =>
      super.copyWith((message) => updates(message as PreviewRequest))
          as PreviewRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PreviewRequest create() => PreviewRequest._();
  @$core.override
  PreviewRequest createEmptyInstance() => create();
  static $pb.PbList<PreviewRequest> createRepeated() =>
      $pb.PbList<PreviewRequest>();
  @$core.pragma('dart2js:noInline')
  static PreviewRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PreviewRequest>(create);
  static PreviewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get diff => $_getSZ(1);
  @$pb.TagNumber(2)
  set diff($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDiff() => $_has(1);
  @$pb.TagNumber(2)
  void clearDiff() => $_clearField(2);
}

class PreviewResponse extends $pb.GeneratedMessage {
  factory PreviewResponse({
    $core.String? preview,
  }) {
    final result = create();
    if (preview != null) result.preview = preview;
    return result;
  }

  PreviewResponse._();

  factory PreviewResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PreviewResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PreviewResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'preview')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewResponse clone() => PreviewResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PreviewResponse copyWith(void Function(PreviewResponse) updates) =>
      super.copyWith((message) => updates(message as PreviewResponse))
          as PreviewResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PreviewResponse create() => PreviewResponse._();
  @$core.override
  PreviewResponse createEmptyInstance() => create();
  static $pb.PbList<PreviewResponse> createRepeated() =>
      $pb.PbList<PreviewResponse>();
  @$core.pragma('dart2js:noInline')
  static PreviewResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PreviewResponse>(create);
  static PreviewResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get preview => $_getSZ(0);
  @$pb.TagNumber(1)
  set preview($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPreview() => $_has(0);
  @$pb.TagNumber(1)
  void clearPreview() => $_clearField(1);
}

class ApplyRequest extends $pb.GeneratedMessage {
  factory ApplyRequest({
    $core.String? sessionId,
    $core.String? diff,
  }) {
    final result = create();
    if (sessionId != null) result.sessionId = sessionId;
    if (diff != null) result.diff = diff;
    return result;
  }

  ApplyRequest._();

  factory ApplyRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ApplyRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplyRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'diff')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplyRequest clone() => ApplyRequest()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplyRequest copyWith(void Function(ApplyRequest) updates) =>
      super.copyWith((message) => updates(message as ApplyRequest))
          as ApplyRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ApplyRequest create() => ApplyRequest._();
  @$core.override
  ApplyRequest createEmptyInstance() => create();
  static $pb.PbList<ApplyRequest> createRepeated() =>
      $pb.PbList<ApplyRequest>();
  @$core.pragma('dart2js:noInline')
  static ApplyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplyRequest>(create);
  static ApplyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get diff => $_getSZ(1);
  @$pb.TagNumber(2)
  set diff($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDiff() => $_has(1);
  @$pb.TagNumber(2)
  void clearDiff() => $_clearField(2);
}

class ApplyResponse extends $pb.GeneratedMessage {
  factory ApplyResponse() => create();

  ApplyResponse._();

  factory ApplyResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ApplyResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ApplyResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'aider.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplyResponse clone() => ApplyResponse()..mergeFromMessage(this);
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ApplyResponse copyWith(void Function(ApplyResponse) updates) =>
      super.copyWith((message) => updates(message as ApplyResponse))
          as ApplyResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ApplyResponse create() => ApplyResponse._();
  @$core.override
  ApplyResponse createEmptyInstance() => create();
  static $pb.PbList<ApplyResponse> createRepeated() =>
      $pb.PbList<ApplyResponse>();
  @$core.pragma('dart2js:noInline')
  static ApplyResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ApplyResponse>(create);
  static ApplyResponse? _defaultInstance;
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');

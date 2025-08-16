import 'dart:io';
import 'package:dio/dio.dart';

final String _baseUrl = Platform.environment['SIDECAR_HTTP'] ?? 'http://localhost:8080';
final String _token = Platform.environment['SIDECAR_TOKEN'] ?? '';
final Dio _dio = Dio(BaseOptions(baseUrl: _baseUrl));

Future<Response<dynamic>> _rpc(String method, Map<String, dynamic>? params) {
  final data = {'method': method, 'params': params ?? {}};
  final options = Options(headers: {
    if (_token.isNotEmpty) 'Authorization': 'Bearer $_token',
  });
  return _dio.post('/rpc', data: data, options: options);
}

Future<List<dynamic>> llmModels() async {
  final resp = await _rpc('llm.models', null);
  final body = resp.data as Map<String, dynamic>;
  if (body['error'] != null) throw Exception(body['error']);
  return body['result'] as List<dynamic>;
}

Future<String> llmChat(List<Map<String, String>> messages) async {
  final resp = await _rpc('llm.chat', {'messages': messages});
  final body = resp.data as Map<String, dynamic>;
  if (body['error'] != null) throw Exception(body['error']);
  return body['result'] as String;
}

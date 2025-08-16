import 'dart:io';
import 'package:dio/dio.dart';

class Analytics {
  final Dio _dio = Dio();
  final String _baseUrl;
  final String? _token;

  Analytics({String? baseUrl})
      : _baseUrl = baseUrl ?? Platform.environment['SIDECAR_HTTP'] ?? 'http://localhost:8080',
        _token = Platform.environment['SIDECAR_TOKEN'];

  Future<void> event(String event, Map<String, dynamic> properties) async {
    await _dio.post('$_baseUrl/rpc',
        data: {
          'method': 'analytics_event',
          'params': {'event': event, 'properties': properties}
        },
        options: Options(headers: {
          if (_token != null && _token!.isNotEmpty) 'Authorization': 'Bearer $_token'
        }));
  }
}

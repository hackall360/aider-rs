import 'package:dio/dio.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class HttpClient {
  final Dio _dio = Dio();

  Future<Response<T>> get<T>(String url) => _dio.get<T>(url);
}

WebSocketChannel connectWebSocket(String url) {
  return WebSocketChannel.connect(Uri.parse(url));
}

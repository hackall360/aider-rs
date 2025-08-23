import 'dart:io';

import 'package:dart_cli/src/http_client.dart';
import 'package:test/test.dart';

void main() {
  test('performs HTTP GET request', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    server.listen((HttpRequest request) {
      request.response
        ..write('ok')
        ..close();
    });

    final client = HttpClient();
    final response = await client
        .get<String>('http://${server.address.host}:${server.port}');
    expect(response.data, 'ok');

    await server.close(force: true);
  });
}

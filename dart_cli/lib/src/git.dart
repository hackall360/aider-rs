import 'dart:async';
import 'package:dart_cli/src/sidecar.dart' as sidecar;

class Git {
  Future<String> run(List<String> args) async {
    final buffer = StringBuffer();
    final code =
        await sidecar.command('git', args, (type, data) => buffer.write(data));
    if (code != 0) {
      throw Exception('git exited with code $code');
    }
    return buffer.toString();
  }
}

import 'dart:io';

import 'package:test/test.dart';

Future<Map<String, String>> _stubCliTui() async {
  final dir = await Directory.systemTemp.createTemp();
  final ext = Platform.isWindows ? '.bat' : '';
  final script = File('${dir.path}/cli_tui$ext');
  if (Platform.isWindows) {
    await script.writeAsString('@echo off\necho %*\n');
  } else {
    await script.writeAsString('#!/usr/bin/env bash\necho "\$@"\n');
    await Process.run('chmod', ['+x', script.path]);
  }
  final env = Map<String, String>.from(Platform.environment);
  final sep = Platform.isWindows ? ';' : ':';
  env['PATH'] = '${dir.path}$sep${env['PATH']}';
  return env;
}

void main() {
  group('dart_cli', () {
    test('passes message to cli_tui', () async {
      final env = await _stubCliTui();
      const msg = 'test message';
      final result = await Process.run(
        'dart',
        ['run', 'bin/dart_cli.dart', '--message', msg],
        environment: env,
      );
      expect(result.exitCode, 0);
      expect(result.stdout.toString().trim(), '--message $msg');
    });

    test('uses default message', () async {
      final env = await _stubCliTui();
      final result = await Process.run(
        'dart',
        ['run', 'bin/dart_cli.dart'],
        environment: env,
      );
      expect(result.exitCode, 0);
      expect(result.stdout.toString().trim(), '--message Hello from Dart');
    });
  });
}

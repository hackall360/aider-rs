import 'dart:io';
import 'package:test/test.dart';
import 'package:dart_cli/src/sidecar.dart';

void main() {
  test('version check via sidecar', () async {
    final process = await Process.start(
        'cargo',
        [
          'run',
          '-p',
          'sidecar',
        ],
        workingDirectory: '../aider-core');
    addTearDown(() => process.kill());
    await Future.delayed(const Duration(seconds: 60));
    final info = await versionCheck();
    expect(info['current'], isNotEmpty);
  }, timeout: const Timeout(Duration(minutes: 2)));
}

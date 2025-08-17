import 'dart:io';

import 'package:args/args.dart';

Future<void> main(List<String> arguments) async {
  final parser = ArgParser()
    ..addOption('message', abbr: 'm', defaultsTo: 'Hello from Dart');
  final results = parser.parse(arguments);
  final msg = results['message'] as String;

  final process = await Process.start('cli_tui', ['--message', msg]);
  await stdout.addStream(process.stdout);
  await stderr.addStream(process.stderr);
  final code = await process.exitCode;
  exit(code);
}

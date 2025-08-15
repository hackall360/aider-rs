import 'dart:async';
import 'dart:io';

import 'package:args/args.dart';
import 'package:io/io.dart';

import 'package:dart_cli/dart_cli.dart';

Future<void> main(List<String> arguments) async {
  final parser = ArgParser()
    ..addFlag('help', abbr: 'h', negatable: false, help: 'Show help')
    ..addOption('config', help: 'Configuration app name');

  final argResults = parser.parse(arguments);

  if (argResults['help'] as bool) {
    stdout.writeln(parser.usage);
    exit(ExitCode.success.code);
  }

  final configName = argResults['config'] as String? ?? 'dart_cli';
  final config = await Config.load(configName);
  stdout.writeln('Loaded config: ${config.data}');

  final git = Git();
  final gitResult = await git.run(['status', '--short']);
  stdout.write(gitResult.stdout);

  final client = HttpClient();
  final response = await client.get<String>('https://example.com');
  stdout.writeln('HTTP status: ${response.statusCode}');

  final channel = connectWebSocket('wss://echo.websocket.events');
  channel.sink.add('ping');
  final wsMessage = await channel.stream.first;
  stdout.writeln('WS event: $wsMessage');
  await channel.sink.close();

  final renderer = TemplateRenderer('Hello {{name}}');
  stdout.writeln(renderer.render({'name': 'World'}));

  // Demonstrate loading shared resources.
  await loadJson('../resources/model-metadata.json');
  await loadYaml('../resources/model-settings.yml');
  await loadPrompt('../resources/prompts/welcome.mustache');
}

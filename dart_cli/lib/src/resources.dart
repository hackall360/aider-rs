import 'dart:convert';
import 'dart:io';

import 'package:toml/toml.dart';
import 'package:yaml/yaml.dart' as yaml;

Future<Map<String, dynamic>> loadJson(String path) async {
  final text = await File(path).readAsString();
  return json.decode(text) as Map<String, dynamic>;
}

Future<Map<String, dynamic>> loadYaml(String path) async {
  final text = await File(path).readAsString();
  final doc = yaml.loadYaml(text);
  return Map<String, dynamic>.from(doc as Map);
}

Future<Map<String, dynamic>> loadToml(String path) async {
  final text = await File(path).readAsString();
  final doc = TomlDocument.parse(text);
  return doc.toMap();
}

Future<String> loadPrompt(String path) async {
  return File(path).readAsString();
}

import 'dart:convert';
import 'dart:io';

import 'package:path/path.dart' as p;
import 'package:toml/toml.dart';
import 'package:xdg_directories/xdg_directories.dart' as xdg;

class Config {
  final Map<String, dynamic> data;

  Config(this.data);

  static Future<Config> load(String appName) async {
    final dir = xdg.configHome.path;
    final jsonPath = p.join(dir, '$appName.json');
    final tomlPath = p.join(dir, '$appName.toml');

    if (await File(jsonPath).exists()) {
      final content = await File(jsonPath).readAsString();
      return Config(jsonDecode(content) as Map<String, dynamic>);
    } else if (await File(tomlPath).exists()) {
      final content = await File(tomlPath).readAsString();
      final doc = TomlDocument.parse(content);
      return Config(doc.toMap());
    } else {
      return Config({});
    }
  }
}

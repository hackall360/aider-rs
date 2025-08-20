import 'dart:io';

import 'package:flutter/material.dart';
import 'package:yaml/yaml.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  final _formKey = GlobalKey<FormState>();
  final _provider = TextEditingController();
  final _tokenBudget = TextEditingController();
  final _chatMode = TextEditingController();
  final _lintCmd = TextEditingController();
  final _testCmd = TextEditingController();
  final _coding = TextEditingController();

  @override
  void initState() {
    super.initState();
    _load();
  }

  void _load() {
    final file = File('.aider.yaml');
    if (!file.existsSync()) return;
    try {
      final doc = loadYaml(file.readAsStringSync());
      if (doc is YamlMap) {
        _provider.text = doc['provider']?.toString() ?? '';
        _tokenBudget.text = doc['token_budget']?.toString() ?? '';
        _chatMode.text = doc['chat_mode']?.toString() ?? '';
        _lintCmd.text = doc['lint_cmd']?.toString() ?? '';
        _testCmd.text = doc['test_cmd']?.toString() ?? '';
        _coding.text = doc['coding_conventions']?.toString() ?? '';
      }
    } catch (_) {}
  }

  Future<void> _save() async {
    if (!_formKey.currentState!.validate()) return;
    final buffer = StringBuffer();
    if (_provider.text.isNotEmpty) {
      buffer.writeln('provider: ${_provider.text}');
    }
    if (_tokenBudget.text.isNotEmpty) {
      buffer.writeln('token_budget: ${_tokenBudget.text}');
    }
    if (_chatMode.text.isNotEmpty) {
      buffer.writeln('chat_mode: ${_chatMode.text}');
    }
    if (_lintCmd.text.isNotEmpty) {
      buffer.writeln('lint_cmd: ${_lintCmd.text}');
    }
    if (_testCmd.text.isNotEmpty) {
      buffer.writeln('test_cmd: ${_testCmd.text}');
    }
    if (_coding.text.isNotEmpty) {
      buffer.writeln('coding_conventions: ${_coding.text}');
    }
    await File('.aider.yaml').writeAsString(buffer.toString());
    if (mounted) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Saved')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: Form(
        key: _formKey,
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            TextFormField(
              controller: _provider,
              decoration: const InputDecoration(labelText: 'Provider'),
            ),
            TextFormField(
              controller: _tokenBudget,
              decoration: const InputDecoration(labelText: 'Token Budget'),
              keyboardType: TextInputType.number,
              validator: (v) {
                if (v == null || v.isEmpty) return null;
                return int.tryParse(v) == null ? 'Enter a number' : null;
              },
            ),
            TextFormField(
              controller: _chatMode,
              decoration: const InputDecoration(labelText: 'Chat Mode'),
            ),
            TextFormField(
              controller: _lintCmd,
              decoration: const InputDecoration(labelText: 'Lint Command'),
            ),
            TextFormField(
              controller: _testCmd,
              decoration: const InputDecoration(labelText: 'Test Command'),
            ),
            TextFormField(
              controller: _coding,
              decoration: const InputDecoration(labelText: 'Coding Conventions'),
            ),
            const SizedBox(height: 16),
            ElevatedButton(onPressed: _save, child: const Text('Save')),
          ],
        ),
      ),
    );
  }
}

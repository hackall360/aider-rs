import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';

import 'grpc_client.dart';
import 'gen/proto/aider.pbgrpc.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  AiderConnection? _conn;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Aider Flutter',
      home: _conn == null
          ? ConnectionPage(onConnected: (c) => setState(() => _conn = c))
          : HomePage(connection: _conn!),
    );
  }
}

class ConnectionPage extends StatefulWidget {
  final ValueChanged<AiderConnection> onConnected;
  const ConnectionPage({super.key, required this.onConnected});

  @override
  State<ConnectionPage> createState() => _ConnectionPageState();
}

class _ConnectionPageState extends State<ConnectionPage> {
  final _controller = TextEditingController(text: 'http://localhost:8080');
  bool _busy = false;
  String? _error;

  Future<void> _connect() async {
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      final conn = await AiderConnection.connect(_controller.text);
      widget.onConnected(conn);
    } catch (e) {
      setState(() {
        _error = '$e';
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Connect')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            TextField(
              controller: _controller,
              decoration: const InputDecoration(labelText: 'Server URL'),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _busy ? null : _connect,
              child: const Text('Connect'),
            ),
            if (_error != null)
              Padding(
                padding: const EdgeInsets.only(top: 8),
                child: Text(_error!, style: const TextStyle(color: Colors.red)),
              ),
          ],
        ),
      ),
    );
  }
}

class HomePage extends StatefulWidget {
  final AiderConnection connection;
  const HomePage({super.key, required this.connection});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _index = 0;

  @override
  Widget build(BuildContext context) {
    final pages = [
      ChatPage(connection: widget.connection),
      FilesPage(connection: widget.connection),
    ];
    return Scaffold(
      body: pages[_index],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _index,
        onTap: (i) => setState(() => _index = i),
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.chat), label: 'Chat'),
          BottomNavigationBarItem(icon: Icon(Icons.folder), label: 'Files'),
        ],
      ),
    );
  }
}

class ChatPage extends StatefulWidget {
  final AiderConnection connection;
  const ChatPage({super.key, required this.connection});

  @override
  State<ChatPage> createState() => _ChatPageState();
}

class _Message {
  _Message(this.role, this.text);
  final String role;
  String text;
}

class _ChatPageState extends State<ChatPage> {
  final List<_Message> _messages = [];
  StreamSubscription? _sub;
  TextEditingController? _inputController;
  static const _commands = ['/add', '/add-url', '/drop', '/help'];

  @override
  void initState() {
    super.initState();
    _sub = widget.connection.ws.stream.listen((event) {
      setState(() {
        if (_messages.isNotEmpty && _messages.last.role == 'assistant') {
          _messages.last.text += event.toString();
        } else {
          _messages.add(_Message('assistant', event.toString()));
        }
      });
    });
  }

  @override
  void dispose() {
    _sub?.cancel();
    super.dispose();
  }

  Future<void> _send() async {
    final text = _inputController?.text ?? '';
    if (text.isEmpty) return;
    _inputController!.clear();
    setState(() {
      _messages.add(_Message('user', text));
      _messages.add(_Message('assistant', ''));
    });
    await widget.connection.session.sendMessage(
      SendMessageRequest(sessionId: widget.connection.sessionId, message: text),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ListView(
            children: _messages
                .map((m) => ListTile(
                      title: Text(m.text),
                      subtitle: Text(m.role),
                    ))
                .toList(),
          ),
        ),
        Row(
          children: [
            Expanded(
              child: Autocomplete<String>(
                optionsBuilder: (value) {
                  if (!value.text.startsWith('/')) {
                    return const Iterable<String>.empty();
                  }
                  return _commands.where((c) => c.startsWith(value.text));
                },
                fieldViewBuilder: (context, controller, focusNode, onFieldSubmitted) {
                  _inputController = controller;
                  return TextField(
                    controller: controller,
                    focusNode: focusNode,
                    onSubmitted: (_) => _send(),
                  );
                },
                onSelected: (s) {
                  _inputController?.text = s;
                },
              ),
            ),
            IconButton(onPressed: _send, icon: const Icon(Icons.send)),
          ],
        ),
      ],
    );
  }
}

class FilesPage extends StatefulWidget {
  final AiderConnection connection;
  const FilesPage({super.key, required this.connection});

  @override
  State<FilesPage> createState() => _FilesPageState();
}

class _FilesPageState extends State<FilesPage> {
  List<_FileInfo> _files = [];
  int _budget = 200;
  int _totalTokens = 0;
  String _filter = '';
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    final resp = await widget.connection.repoMap.getMap(
      GetMapRequest(
          sessionId: widget.connection.sessionId, tokenBudget: _budget),
    );
    final data = jsonDecode(resp.mapJson);
    final List<_FileInfo> files = [];
    if (data is Map) {
      _totalTokens = data['total_tokens'] ?? 0;
      if (data['files'] is List) {
        for (var f in data['files']) {
          if (f is Map) {
            final symbols = <_SymbolInfo>[];
            if (f['symbols'] is List) {
              for (var s in f['symbols']) {
                if (s is Map && s['name'] is String) {
                  symbols.add(_SymbolInfo(
                      name: s['name'] as String,
                      line: s['line'] ?? 0,
                      relevance: (s['relevance'] ?? 0).toDouble(),
                      tokens: s['tokens'] ?? 0));
                }
              }
            }
            files.add(_FileInfo(
                path: f['path'] ?? '',
                relevance: (f['relevance'] ?? 0).toDouble(),
                tokens: f['tokens'] ?? 0,
                symbols: symbols));
          }
        }
      }
    }
    setState(() {
      _files = files;
      _loading = false;
    });
  }

  List<_FileInfo> get _visibleFiles {
    if (_filter.isEmpty) return _files;
    return _files
        .map((f) => _FileInfo(
            path: f.path,
            relevance: f.relevance,
            tokens: f.tokens,
            symbols: f.symbols
                .where((s) => s.name.toLowerCase().contains(_filter))
                .toList()))
        .where((f) => f.symbols.isNotEmpty)
        .toList();
  }

  Future<void> _showSnippet(_FileInfo file, _SymbolInfo sym) async {
    final resp = await widget.connection.repoMap.getSnippet(
      SnippetRequest(
          sessionId: widget.connection.sessionId,
          path: file.path,
          line: sym.line,
          context: 2),
    );
    if (!mounted) return;
    await showModalBottomSheet(
      context: context,
      builder: (context) {
        return Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(resp.content, style: const TextStyle(fontFamily: 'monospace')),
              const SizedBox(height: 16),
              Align(
                alignment: Alignment.centerRight,
                child: ElevatedButton(
                  onPressed: () async {
                    await _addFile(file.path);
                    if (context.mounted) Navigator.pop(context);
                  },
                  child: const Text('Add to context'),
                ),
              )
            ],
          ),
        );
      },
    );
  }

  Future<void> _addFile(String path) async {
    final files = [SetFilesRequest_File(path: path, content: '')];
    await widget.connection.session.setFiles(
      SetFilesRequest(sessionId: widget.connection.sessionId, files: files),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_loading) {
      return const Center(child: CircularProgressIndicator());
    }
    final overBudget = _totalTokens > _budget;
    final files = _visibleFiles;
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(8),
          child: TextField(
            decoration: const InputDecoration(labelText: 'Filter symbols'),
            onChanged: (v) => setState(() => _filter = v.toLowerCase()),
          ),
        ),
        if (overBudget)
          const Text('Over budget', style: TextStyle(color: Colors.red)),
        Slider(
          value: _budget.toDouble(),
          min: 50,
          max: 1000,
          divisions: 19,
          label: '$_budget',
          onChanged: (v) => setState(() => _budget = v.toInt()),
          onChangeEnd: (v) => _load(),
        ),
        Expanded(
          child: ListView(
            children: files
                .map(
                  (f) => ExpansionTile(
                    title: Text(f.path),
                    subtitle: Row(
                      children: [
                        Chip(label: Text('r ${f.relevance.toStringAsFixed(2)}')),
                        const SizedBox(width: 8),
                        Chip(label: Text('${f.tokens}t')),
                      ],
                    ),
                    children: f.symbols
                        .where((s) =>
                            _filter.isEmpty ||
                            s.name.toLowerCase().contains(_filter))
                        .map(
                          (s) => ListTile(
                            title: Text(s.name),
                            trailing: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Chip(label: Text('r ${s.relevance.toStringAsFixed(2)}')),
                                const SizedBox(width: 8),
                                Chip(label: Text('${s.tokens}t')),
                              ],
                            ),
                            onTap: () => _showSnippet(f, s),
                          ),
                        )
                        .toList(),
                  ),
                )
                .toList(),
          ),
        ),
      ],
    );
  }
}

class _FileInfo {
  final String path;
  final double relevance;
  final int tokens;
  final List<_SymbolInfo> symbols;
  _FileInfo({required this.path, required this.relevance, required this.tokens, required this.symbols});
}

class _SymbolInfo {
  final String name;
  final int line;
  final double relevance;
  final int tokens;
  _SymbolInfo({required this.name, required this.line, required this.relevance, required this.tokens});
}

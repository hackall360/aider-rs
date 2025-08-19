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
  static const _commands = ['/add', '/drop', '/help'];

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
  late Future<List<String>> _files;
  final Set<String> _selected = {};

  @override
  void initState() {
    super.initState();
    _files = _load();
  }

  Future<List<String>> _load() async {
    final resp = await widget.connection.repoMap.getMap(
      GetMapRequest(sessionId: widget.connection.sessionId),
    );
    final data = jsonDecode(resp.mapJson);
    final List<String> out = [];
    if (data is Map && data['files'] is List) {
      for (var f in data['files']) {
        if (f is String) {
          out.add(f);
        } else if (f is Map && f['path'] is String) {
          out.add(f['path'] as String);
        }
      }
    }
    return out;
  }

  Future<void> _toggle(String path, bool checked) async {
    setState(() {
      if (checked) {
        _selected.add(path);
      } else {
        _selected.remove(path);
      }
    });
    final files = _selected
        .map((p) => SetFilesRequest_File(path: p, content: ''))
        .toList();
    await widget.connection.session.setFiles(
      SetFilesRequest(sessionId: widget.connection.sessionId, files: files),
    );
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<String>>(
      future: _files,
      builder: (context, snapshot) {
        if (!snapshot.hasData) {
          return const Center(child: CircularProgressIndicator());
        }
        final files = snapshot.data!;
        return ListView(
          children: files
              .map(
                (f) => CheckboxListTile(
                  title: Text(f),
                  value: _selected.contains(f),
                  onChanged: (v) => _toggle(f, v ?? false),
                ),
              )
              .toList(),
        );
      },
    );
  }
}

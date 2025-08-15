import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'api.dart' as api;
import 'chat_database.dart';
import 'frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  final db = await ChatDatabase.open();
  runApp(MyApp(db: db));
}

class MyApp extends StatelessWidget {
  final ChatDatabase db;
  const MyApp({super.key, required this.db});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Aider Flutter',
      home: HomePage(db: db),
    );
  }
}

class HomePage extends StatefulWidget {
  final ChatDatabase db;
  const HomePage({super.key, required this.db});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final dio = Dio();
  late final WebSocketChannel channel;
  int _index = 0;

  @override
  void initState() {
    super.initState();
    channel = WebSocketChannel.connect(
      Uri.parse('wss://echo.websocket.events'),
    );
    channel.stream.listen((event) {
      // handle incoming messages
    });
  }

  @override
  void dispose() {
    channel.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final pages = [
      ChatPage(db: widget.db, dio: dio, channel: channel),
      const RepoNavigatorPage(),
    ];
    return Scaffold(
      body: pages[_index],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _index,
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.chat), label: 'Chat'),
          BottomNavigationBarItem(icon: Icon(Icons.folder), label: 'Repos'),
        ],
        onTap: (i) => setState(() => _index = i),
      ),
    );
  }
}

class ChatPage extends StatefulWidget {
  final ChatDatabase db;
  final Dio dio;
  final WebSocketChannel channel;

  const ChatPage({
    super.key,
    required this.db,
    required this.dio,
    required this.channel,
  });

  @override
  State<ChatPage> createState() => _ChatPageState();
}

class _ChatPageState extends State<ChatPage> {
  final controller = TextEditingController();
  final messages = <String>[];

  Future<void> _send() async {
    final text = controller.text;
    controller.clear();
    final response = await api.llm(prompt: text);
    widget.db.addMessage(text, response);
    setState(() {
      messages.add('Me: ' + text);
      messages.add('Rust: ' + response);
    });
    widget.channel.sink.add(text);
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ListView.builder(
            itemCount: messages.length,
            itemBuilder: (context, i) => ListTile(title: Text(messages[i])),
          ),
        ),
        Row(
          children: [
            Expanded(child: TextField(controller: controller)),
            IconButton(icon: const Icon(Icons.send), onPressed: _send),
          ],
        ),
      ],
    );
  }
}

class RepoNavigatorPage extends StatelessWidget {
  const RepoNavigatorPage({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: api.repoMap(),
      builder: (context, snapshot) {
        return Center(
          child: Text(snapshot.data ?? 'Loading...'),
        );
      },
    );
  }
}

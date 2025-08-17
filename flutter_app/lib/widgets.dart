import 'package:flutter/material.dart';

import 'chat_database.dart';

class ChatHistory extends StatelessWidget {
  final ChatDatabase db;
  const ChatHistory({super.key, required this.db});

  @override
  Widget build(BuildContext context) {
    final messages = db.getMessages();
    return ListView.builder(
      itemCount: messages.length,
      itemBuilder: (context, i) {
        final msg = messages[i];
        return ListTile(
          title: Text('Me: ${msg["prompt"]}'),
          subtitle: Text('Aider: ${msg["response"]}'),
        );
      },
    );
  }
}

class FileDiffViewer extends StatelessWidget {
  final String diff;
  const FileDiffViewer({super.key, required this.diff});

  @override
  Widget build(BuildContext context) {
    if (diff.isEmpty) return const SizedBox();
    return Container(
      color: Colors.black12,
      height: 150,
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(8),
        child: Text(diff, style: const TextStyle(fontFamily: 'monospace')),
      ),
    );
  }
}

class WebContentInput extends StatefulWidget {
  final Future<void> Function(String url) onSubmit;
  const WebContentInput({super.key, required this.onSubmit});

  @override
  State<WebContentInput> createState() => _WebContentInputState();
}

class _WebContentInputState extends State<WebContentInput> {
  final controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: TextField(
            controller: controller,
            decoration: const InputDecoration(labelText: 'URL'),
          ),
        ),
        IconButton(
          icon: const Icon(Icons.download),
          onPressed: () => widget.onSubmit(controller.text),
        ),
      ],
    );
  }
}

class UndoButton extends StatelessWidget {
  final VoidCallback onPressed;
  const UndoButton({super.key, required this.onPressed});

  @override
  Widget build(BuildContext context) {
    return IconButton(
      icon: const Icon(Icons.undo),
      onPressed: onPressed,
    );
  }
}


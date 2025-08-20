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

class CommitMessageDialog extends StatefulWidget {
  final String initialMessage;
  final void Function(String message) onAccept;
  const CommitMessageDialog(
      {super.key, required this.initialMessage, required this.onAccept});

  @override
  State<CommitMessageDialog> createState() => _CommitMessageDialogState();
}

class _CommitMessageDialogState extends State<CommitMessageDialog> {
  late TextEditingController controller;

  @override
  void initState() {
    super.initState();
    controller = TextEditingController(text: widget.initialMessage);
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Commit Message'),
      content: TextField(
        controller: controller,
        maxLines: 3,
        decoration: const InputDecoration(border: OutlineInputBorder()),
      ),
      actions: [
        TextButton(
          onPressed: () => setState(() {}),
          child: const Text('Edit'),
        ),
        TextButton(
          onPressed: () => widget.onAccept(controller.text),
          child: const Text('Accept'),
        ),
      ],
    );
  }
}


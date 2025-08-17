import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';
import 'package:sqlite3/sqlite3.dart';

class ChatDatabase {
  final Database _db;

  ChatDatabase._(this._db);

  static Future<ChatDatabase> open() async {
    final dir = await getApplicationDocumentsDirectory();
    final path = p.join(dir.path, 'chat.db');
    final db = sqlite3.open(path);
    db.execute('''
      CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        prompt TEXT NOT NULL,
        response TEXT NOT NULL
      )
    ''');
    return ChatDatabase._(db);
  }

  void addMessage(String prompt, String response) {
    _db.execute('INSERT INTO messages (prompt, response) VALUES (?, ?)', [
      prompt,
      response,
    ]);
  }

  List<Map<String, String>> getMessages() {
    final rs =
        _db.select('SELECT prompt, response FROM messages ORDER BY id');
    return rs
        .map((row) => {
              'prompt': row['prompt'] as String,
              'response': row['response'] as String,
            })
        .toList();
  }

  void deleteLastMessage() {
    _db.execute(
        'DELETE FROM messages WHERE id = (SELECT MAX(id) FROM messages)');
  }
}

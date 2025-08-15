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
}

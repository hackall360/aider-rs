import 'dart:io';

import 'package:process_run/process_run.dart' as pr;

class Git {
  Future<ProcessResult> run(List<String> args, {String? workingDirectory}) {
    return pr.runExecutableArguments('git', args,
        workingDirectory: workingDirectory);
  }
}

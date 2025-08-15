import 'package:mustache_template/mustache_template.dart';

class TemplateRenderer {
  final Template _template;

  TemplateRenderer(String source) : _template = Template(source, lenient: true);

  String render(Map<String, Object?> data) => _template.renderString(data);
}

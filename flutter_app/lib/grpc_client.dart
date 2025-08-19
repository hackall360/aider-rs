import 'package:grpc/grpc.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'gen/proto/aider.pbgrpc.dart';

/// Helper class managing gRPC and WebSocket connections to the aider server.
class AiderConnection {
  final ClientChannel channel;
  final SessionServiceClient session;
  final RepoMapServiceClient repoMap;
  final DiffServiceClient diff;
  final String sessionId;
  final WebSocketChannel ws;

  AiderConnection._(
    this.channel,
    this.session,
    this.repoMap,
    this.diff,
    this.sessionId,
    this.ws,
  );

  /// Connect to the aider server at [url].
  ///
  /// This opens a gRPC channel and session and establishes a WebSocket
  /// connection for streaming tokens.
  static Future<AiderConnection> connect(String url) async {
    final uri = Uri.parse(url);
    final channel = ClientChannel(
      uri.host,
      port: uri.port,
      options: ChannelOptions(
        credentials: uri.scheme == 'https'
            ? const ChannelCredentials.secure()
            : const ChannelCredentials.insecure(),
      ),
    );

    final sessionClient = SessionServiceClient(channel);
    final repoClient = RepoMapServiceClient(channel);
    final diffClient = DiffServiceClient(channel);

    final openResp = await sessionClient.open(
      OpenRequest(clientVersion: 'flutter'),
    );

    final wsScheme = uri.scheme == 'https' ? 'wss' : 'ws';
    final wsUrl = Uri.parse('$wsScheme://${uri.authority}/ws/${openResp.sessionId}');
    final ws = WebSocketChannel.connect(wsUrl);

    return AiderConnection._(
      channel,
      sessionClient,
      repoClient,
      diffClient,
      openResp.sessionId,
      ws,
    );
  }

  /// Close both the gRPC channel and WebSocket connection.
  Future<void> close() async {
    await ws.sink.close();
    await channel.shutdown();
  }
}

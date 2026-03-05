import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:matrix_rust_sdk_bridge/matrix_rust_sdk_bridge.dart';

late Stream<String> events;
late Stream<List<RoomEntity>> sEvent;

late String homeserverUrl;
late String dbPath;
late String passPhrase;
late String accessToken;
late String deviceId;
late String userId;
late String username;
late String password;
late String accountId;

late UserSessionEntity user_session;
late ClientSessionEntity client_session;
late Credentials credentials;

Future<void> main() async {
  await dotenv.load(fileName: ".env");

  homeserverUrl = dotenv.env['homeserver_url'] ?? '';
  //final appDir = await getApplicationDocumentsDirectory();
  //dbPath = '${appDir.path}/${dotenv.env['db_dir'] ?? ''}';
  dbPath = dotenv.env['db_dir'] ?? '';
  passPhrase = dotenv.env['passphrase'] ?? '';
  username = dotenv.env['username'] ?? '';
  password = dotenv.env['password'] ?? '';
  accessToken = dotenv.env['MATRIX_ACCESSTOKEN'] ?? '';
  deviceId = dotenv.env['MATRIX_DEVICEID'] ?? '';
  userId = dotenv.env['MATRIX_USER_ID'] ?? '';
  accountId = dotenv.env['ACCOUNT_ID'] ?? '';

  debugPrint(homeserverUrl + dbPath + passPhrase);

  user_session = UserSessionEntity(
    accessToken: accessToken,
    deviceId: deviceId,
    matrixUserId: userId,
  );

  client_session = ClientSessionEntity(
    homeserver: homeserverUrl,
    sessionPath: dbPath,
    passphrase: passPhrase,
  );

  credentials = Credentials.userPassword(
    username: username,
    password: password,
  );

  await RustLib.init();

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(home: const MatrixHomePage());
  }
}

class MatrixHomePage extends StatefulWidget {
  const MatrixHomePage({super.key});

  @override
  State<MatrixHomePage> createState() => _MatrixHomePageState();
}

class _MatrixHomePageState extends State<MatrixHomePage> {
  bool _isInitialized = false;

  void _onInitialized() {
    setState(() {
      _isInitialized = true;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Matrix Messages')),
      body: _isInitialized
          ? const MessageListWidget()
          : InitializationWidget(onInitialized: _onInitialized),
    );
  }
}

class InitializationWidget extends StatelessWidget {
  final VoidCallback onInitialized;

  const InitializationWidget({super.key, required this.onInitialized});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: registerMatrixClient(
        session: MatrixSessionEntity(
          clientSession: client_session,
          userSession: user_session, // Optional field
          syncToken: null, // Optional field
          credentials: credentials,
        ),
      ),
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Center(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Icon(Icons.error, color: Colors.red, size: 48),
                  const SizedBox(height: 16),
                  Text(
                    "Error Initializing Matrix",
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    "${snapshot.error}",
                    style: const TextStyle(color: Colors.red),
                    textAlign: TextAlign.center,
                  ),
                ],
              ),
            ),
          );
        }
        if (snapshot.hasData) {
          final result = snapshot.data;
          if (result != null) {
            debugPrint(result.userSession?.accessToken);
            // Call the callback to switch to the message widget
            WidgetsBinding.instance.addPostFrameCallback((_) async {
              // Delay 2 seconds
              await Future.delayed(const Duration(milliseconds: 500));
              onInitialized();
            });
            return const Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.check_circle, color: Colors.green, size: 48),
                  SizedBox(height: 16),
                  Text('Matrix Client Initialized'),
                ],
              ),
            );
          }
        }
        return const Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              CircularProgressIndicator(),
              SizedBox(height: 16),
              Text('Initializing Matrix Client...'),
            ],
          ),
        );
      },
    );
  }
}

class MessageListWidget extends StatefulWidget {
  const MessageListWidget({super.key});

  @override
  State<MessageListWidget> createState() => _MessageListWidgetState();
}

class _MessageListWidgetState extends State<MessageListWidget> {
  final List<String> _messages = [];
  bool _isSyncing = false;

  void _addMessage(String eventText) {
    try {
      setState(() {
        _messages.insert(0, eventText);
      });
    } catch (e) {
      debugPrint('Error parsing message: $e');
    }
  }

  Future<void> _handleStartSync() async {
    try {
      setState(() {
        _isSyncing = true;
      });
      events = syncEvents(accountId: accountId);
      events.listen(
        (json) {
          _addMessage(json);
        },
        onError: (error) {
          debugPrint('Sync error: $error');
          if (mounted) {
            setState(() {
              _isSyncing = false;
            });
          }
        },
      );
    } catch (error) {
      debugPrint(error.toString());
      setState(() {
        _isSyncing = false;
      });
    }
  }

  void _handleClear() {
    setState(() {
      _messages.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: Card(
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Column(
                children: [
                  Wrap(
                    spacing: 8,
                    runSpacing: 8,
                    children: [
                      ElevatedButton.icon(
                        onPressed: () async {
                          sEvent = syncRoomsBySpace(
                            spaceId: "hello_world",
                            accountId: accountId,
                          );
                          sEvent.listen(
                            (roomEntity) {
                              debugPrint('Room event: ${roomEntity.length}');
                            },
                            onError: (error) {
                              debugPrint('Room Service error: $error');
                            },
                          );
                        },
                        icon: const Icon(Icons.sync_alt, size: 18),
                        label: const Text("get rooms"),
                      ),
                      ElevatedButton.icon(
                        onPressed: _isSyncing ? null : _handleStartSync,
                        icon: _isSyncing
                            ? const SizedBox(
                                width: 18,
                                height: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                ),
                              )
                            : const Icon(Icons.sync, size: 18),
                        label: Text(_isSyncing ? "Syncing..." : "Start Sync"),
                      ),
                      if (_messages.isNotEmpty)
                        ElevatedButton.icon(
                          onPressed: _handleClear,
                          icon: const Icon(Icons.clear_all, size: 18),
                          label: const Text("Clear"),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.red,
                            foregroundColor: Colors.white,
                          ),
                        ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Icon(Icons.message, size: 16),
                      const SizedBox(width: 8),
                      Text(
                        'Messages: ${_messages.length}',
                        style: Theme.of(context).textTheme.titleMedium,
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
        const Divider(height: 1),
        Expanded(
          child: _messages.isEmpty
              ? const Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.inbox, size: 64, color: Colors.grey),
                      SizedBox(height: 16),
                      Text(
                        'No messages yet',
                        style: TextStyle(color: Colors.grey, fontSize: 16),
                      ),
                      SizedBox(height: 8),
                      Text(
                        'Start sync to see messages',
                        style: TextStyle(color: Colors.grey, fontSize: 14),
                      ),
                    ],
                  ),
                )
              : ListView.builder(
                  itemCount: _messages.length,
                  itemBuilder: (context, index) {
                    final message = _messages[index];
                    return Card(
                      margin: const EdgeInsets.symmetric(
                        horizontal: 8,
                        vertical: 4,
                      ),
                      child: ExpansionTile(
                        title: Text(
                          'Event $message',
                          style: const TextStyle(fontWeight: FontWeight.bold),
                        ),
                      ),
                    );
                  },
                ),
        ),
      ],
    );
  }
}

import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:matrix_rust_sdk_bridge/matrix_rust_sdk_bridge.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  test('Can call rust function', () async {
    //expect(greet(name: "Tom"), "Hello, Tom!");
  });
}

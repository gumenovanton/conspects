// combine two streams into one with equal types
// and returns values in order one after another

import 'package:rxdart/rxdart.dart';

void main() {
  // Example usage:
  final stream1 = Stream.fromIterable([1, 2, 3]);
  final stream2 = Stream.fromIterable([4, 5, 6]);

  final combinedStream = Rx.concat<int>([stream1, stream2]);

  combinedStream.listen(print); // 1, 2, 3, 4, 5, 6
}

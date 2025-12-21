// method to combine up to 9 streams with a different initial value for each stream
// used to combine many streams and format value for the view, for example return a map of values

import 'package:rxdart/rxdart.dart';

void main() {
  final sityStream = Stream.fromIterable(['London', 'Paris', 'Berlin']);
  final streetStream = Stream.fromIterable(['Street1', 'Street2', 'Street3']);
  final houseStream = Stream.fromIterable([7, 8, 9]);

  // when i need to combine 3 streams
  // new value generates every time when any of the streams emits a new value
  final generateMap = Rx.combineLatest3(
    sityStream,
    streetStream,
    houseStream,
    (String a, String b, int c) => {'sity': a, 'street': b, 'house': c},
  );

  generateMap.listen(print);
}

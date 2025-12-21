// one of the least used ones

import 'package:rxdart/rxdart.dart';

void main() {
  // ReplaySubject create stream that emits the last n events
  final replaySubject = ReplaySubject<int>(maxSize: 3);
  replaySubject.add(1);
  replaySubject.add(2);
  replaySubject.add(3);
  replaySubject.add(4);
  replaySubject.add(5);
  replaySubject.add(6);
  replaySubject.add(7);

  // will receive 5, 6, 7
  replaySubject.listen((value) => print(value));

  replaySubject.close();
}

import 'package:rxdart/rxdart.dart';

void main() {
  // BEHAVIOR SUBJECT --------------------------------------------------------------------------------------------------------------------
  // creates a stream that emits the last value and all subsequent values
  // the most used one
  final behaviorSubject = BehaviorSubject<int>.seeded(0);

  behaviorSubject.add(1);
  behaviorSubject.add(2);

  // will receive  only 2, 3, 4, 5
  final subscriberOne = behaviorSubject.listen((value) {
    print('Subscriber One: $value');
  });

  behaviorSubject.add(3);
  behaviorSubject.add(4);
  behaviorSubject.add(5);

  // will receive only 5
  final subscriberTwo = behaviorSubject.listen((value) {
    print('Subscriber Two: $value');
  });

  //i hadn't wait for the stream to complete before closing it
  behaviorSubject.close();
}

// just like a simple standart dart stream

import 'package:rxdart/rxdart.dart';

void main() {
  final publishSubject = PublishSubject<int>();
  publishSubject.add(1);

  //will receive values: 2,3,4
  publishSubject.listen((value) => print('Received value: $value'));

  publishSubject.add(2);
  publishSubject.add(3);
  publishSubject.add(4);

  publishSubject.close();
}

// STREAMS

import 'dart:async';

void main() {
  // SIMPLE STREAM

  // Create a stream of integers from 1 to 10
  Stream<int> stream = Stream.fromIterable([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

  // Listen to the stream and print each value
  final subscription = stream.listen((value) {
    print('simple stream: $value');
  });

  // streams a async, ind i need to wait for the stream to finish before canceling the subscription
  Future.delayed(Duration(seconds: 1), () {
    subscription.cancel();
  });

  // STREAM CONTROLLER --------------------------------------------------------------------------------------------------------------------
  // to be more useful streams wrapped with a StreamController type
  // this stream controller can listen only one listener at a time

  // Create a stream controller
  StreamController<int> controller = StreamController<int>();

  // Add values to the stream controller
  controller.add(1);
  controller.add(2);
  controller.add(3);

  // Listen to the stream controller and print each value
  controller.stream.listen((value) {
    print('stream controller: $value');
  });

  // Every stream must be closed when it is no longer needed
  controller.close();

  // BROADCAST STREAM  --------------------------------------------------------------------------------------------------------------------
  // this stream controller can listen multiple listeners at a time
  // but one can lose data

  // Create a broadcast stream controller
  StreamController<int> broadcastController = StreamController<int>.broadcast();

  // Listen to the broadcast stream controller and print each value
  final subscription1 = broadcastController.stream.listen((value) {
    print('subscripcion1: $value');
  });

  // Add values to the broadcast stream controller
  broadcastController.add(1);
  broadcastController.add(2);
  broadcastController.add(3);

  // catch no data
  final subscription2 = broadcastController.stream.listen((value) {
    print('subscripcion2: $value');
  });

  // or cancel the subscription when it is no longer needed
  Future.delayed(Duration(seconds: 2), () {
    subscription1.cancel();
    subscription2.cancel();
  });

  // Every stream must be closed when it is no longer needed
  broadcastController.close();
}

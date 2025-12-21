// wait for all streams to emit a value to combine them into a single value

import 'package:rxdart/rxdart.dart';

void main() {
  // Create two streams
  final sityStream = Stream.fromIterable([
    'New York',
    'Sacramento',
    'Boston',
    'Chicago',
    'Miami',
  ]);
  final teamNameStream = Stream.fromIterable(['Knicks', 'Kings', 'Celtics']);

  // Chicago and Miami will not be combined with any team name
  final combinedStream = Rx.zip2(
    sityStream,
    teamNameStream,
    (city, teamName) => '$city $teamName',
  );

  // Listen to the combined stream and print the combined values
  combinedStream.listen((combinedValues) {
    print(combinedValues);
  });
}

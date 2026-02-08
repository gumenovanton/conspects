// FREEZED:
// code generation for models such as
// - copyWith
// - fields
// - == operator
// - fromJson
// - toJson
// - etc

// INSTALATION:
// flutter pub add dev:build_runner freezed_annotation dev:freezed

// if using freezed to generate fromJson/toJson, also add:
// flutter pub add json_annotation dev:json_serializable

// HOW_TO_USE:
// STEP 1: - create a file name for example user.dart

// STEP 2: - add import of freezed_annotation to the top of the file
import 'package:freezed_annotation/freezed_annotation.dart';

// STEP 3: - add imports to generate the file
// it necessary that name of the file equals import names for example if your file name is user.dart
part 'user.freezed.dart'; // - this import for generation

// STEP 4: - add a model in the file
// add a const factory constructor with @freesed annotation
// and redirect it
@freezed
class User with _@User{
 const foctory User({
  required String name,
  int? age,
 }) = _User;
}

// STEP 5: - run code generation
flutter pub run build-runner watch --delete-conflicting-outputs

// STEP 6: - to disable linter warnings add this lines into analysis_options.yaml
analyzer:
  exclude;
    - '**/*.g.dart'
    - '**/*.freezed.dart'
  errors:
    invalid_annotation_target: ignore



// FREEZED:
// code generation for models such as
// - copyWith
// - fields
// - == operator
// - fromJson
// - toJson
// - etc

// INSTALATION:
flutter pub add dev:build_runner freezed_annotation dev:freezed

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
abstract class User with _@User{
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

// HOW_TO_ADD_JSON_SERIALIZATION:
// everything after steps above
// it adds toJson and fromJson functionallity

// STEP 1: - add dependencies
flutter pub add json_annotation dev:json_serializable

// STEP 2: - this in user.dart model file
part 'model_name.g.dart'; 

// STEP 3: - add new factory in the model.dart
factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);

// HOW_TO_ADD_CUSTOM_METHODS:
// STEP 1: - add a constructor into model
cost User._();

// STEP 2: - add a custom method into model
String forFun()=> name * age;

// HOW_ADD_INNER_SERIALIZABLE_MODEL:
// if i add to User an inner Model field like
required List<Job> jobs,

// to serialize and deserialize it i need to create a Job model although with freezed
@freezed
class Job with _$Job{
    const factory Job({
        // here example with default field
        @Default('some value') String title,
        required int level,
    }) = _Job;

    // add a factory constructor fromJson
    factory Job.fromJson(Map<String, dynamic> json)=> _$JobFromJson(json);
}

// if i do something like this
conts userA = User(
    name: 'User A',
    age: 20,
    jobs:[
        Job(level:3);
    ]
);

// add @JsonSErializable annotation to User
class User with _@User{
    @JsonSErializable(explictToJson: true)
    const foctory User({
      required String name,
      required List<Job> jobs,
      int? age,
    }) = _User;

    factory User.fromJson(Map<String, dynamic> json)=> _$UserFromJson(json);
}

// and i can toJson with it
print(userA.toJson());

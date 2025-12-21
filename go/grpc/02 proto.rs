# PROTOBUF
// its a contract between the server an the client
// its discribes what data will be send, and what the procedures will be called
// the servers proto file must be equal the client proto file

# DATA TYPES
//  PROTO       GO
//  double      float32
//  float       float64
//  int32       int32
//  int64       int64
//  uint32      uint32
//  uint64      uint64
//  sint32      int32 - uptimized for negative numbers
//  sint64      int32 - uptimized for negative numbers
//  fixed32     fixed32 - uptimized for big numbers
//  fixed64     fixed64 - uptimized for big numbers
//  bool, string
//  bytes       []byte - for files
//  golang nulable type - all types can be this type, it means no value

// google.protobuf.Timestamp - one of nulable type

# CREATING A MESSAGES
// i must define index of a field, because the gRPC do not send field names
message InfoResponse{
    int64 id = 1;
    int64 moderator_id = 2;
    google.protobuf.Timestamp start_at = 3;
    google.protobuf.Timestamp last_active_at = 4;
    repeated int64 task_ids = 5; // array
}

# CREATING AN ENUMS
// god practice the 0 index define like NOT_HEFINED
// and i can reserve some indexes and ranges for future
// to don't brake your app
enum ContentType{
    POST = 0;
    COMENT = 1;
    reserved 2 to 7;
    STREAM = 8;
    reserved 19 to 27;
}

# CREATE INLINE MESSAGES
message NoteContent{
    string title = 1;
    string text = 2;
}

message CreateNoteRequest{
    NoteContent content = 1;
}

# CREATING A SERVICE
service UserV1{
    rpc Get(GetRequest) returns(GetResponse)
}

# EXAMPLE
syntax = "proto3";

package user_v1; // package to have access to another proto files

// used only in go
option go_package = "package name/pkg/user_v1/;user_v1" // where to generate
// package name/pkg/user_v1 - package name from go mod
// user_v1 - package name in generated files

message UserInfo{
    int64 id = 1;
    string name = 2;
    bool is_human = 3;
}

message GetRequest{
    int64 id = 1;
}

message GetResponse{
    UserInfo info = 1;
}
// a grpc endpoint can get and return only one param
service UserV1{
    rpc Get(GetRequest) returns(GetResponse)
}

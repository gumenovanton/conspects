# OPPORTUNITIES
//<< grpc helps to implement 4 types of client-server communication models
// - unaru - request and response, like rest
// - server streaming - the client sends a request, and the server sends a data stream
// - client streaming - the client sends a data stream, and the server sends a response
// - two way streaming - the client and the server sends data streams to each other

// - with grpc i can do authentication

# WORKFLOW
<| STEP 1:
// all protofiles in progect stores in a separate github or gitlab repo
// - i need to create a new repo and create a folders tree like this
//      protos
//      |-- gen
//      |   |-- go
//      |   |   |--- different_projects_folders - here will live all generated go files by projects
//      |-- proto
//      |   |-- different_projects_folders - here will live all proto files by projects

// - init a go module in it
// for example i has a project flutter + go-backend
// i need to has three foulders
// - front
// - back
// - protos (because both use protos)



<| STEP 2: create proto file
// - i must define messages(like structs)
// and services(like api)

<| STEP 3: generate client and server stubs
// install protoc
sudo pamac install protobuf
// binaries for go generation
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

protoc -I proto proto/auth/auth.proto --go_out=./gen/go --go_opt=paths=source_relative --go-grpc_out=./gen/go --go-grpc_opt=paths=source_relative

// - -I proto - the path to foulder where all protofiles will live,
// - proto/auth/auth.proto - the path to the proto file
// - --go_out=./gen/go - where to put genereted files
// - --go_opt=paths=source_relative - it tels the protoc that generated files will live in folder named equal with proto files foulder name
// - --go-grpc_out=./gen/go - where to put genereted grpc files
// - --go-grpc_opt=paths=source_relative - it tels the protoc that generated files will have the same package that protofiles

<| STEP 4: implement client and server mehods

//>> jsoniter
// fastest json marshaler

//<< install
go get github.com/json-iterator/go

//<< usage
var v SomeType

b, err := io.ReadAll(r)
err = jsoniter.Unmarshal(jsonString, &v)

jsonString, err := jsoniter.Marshal(v)
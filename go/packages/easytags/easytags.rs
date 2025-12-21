//>> easytags
// to generate tags for structure

//<< install
go install github.com/betacraft/easytags@latest

//>> usage in code
//<< write go generate comment
//go:generate easytags $GOFILE json,xml,sql
type User struct{
    Name string 
}

//<< generate
go generate

//>> installation
go install github.com/swaggo/swag/cmd/swag@latest

//>> usage
//<< step 1: write annotations on handlerFunc
// @Summary AddressSearcb
// @Security ApiKeyAuth
// @Tags search
// @Description get address suggestions from address string
// @ID search
// @Produce json
// @Param input body RequestAddressSearch true "address string"
// @Success 200 {array} ResponseAddress
// @Failture 400 responder.Response
// @Failture 500 responder.Response
// @Router /api/address/search [post]

//<< step 2: write annotations no main func
// @title Geo Service API
// @version 1.0
// @description You can get address suggestion by address strind.

// @host localhost:8080
// @BasePath /

// @SecurityDefinitions.apikey ApiKeyAuth
// @In header
// @Name Authorization

//<< step 3: run swag init
// this command creates docs directory
swag init -g ./cmd/api/main.go
// or
swag init -dir ./cmd/api

//<< step 4: install httpSwagger
go get -u github.com/swaggo/http-swagger

//<< step 5: import swag docs to router file, and add swag route to router
import (
    _ "geo/docs"

	httpSwagger "github.com/swaggo/http-swagger"
)

r.Get("/swagger/*", httpSwagger.Handler(
    httpSwagger.URL("http://localhost:8080/swagger/doc.json"), //The url pointing to API definition
))

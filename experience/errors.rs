# TIPS
// never return to user an error detales
// if wrong pass or login just return wrong credentials

## where to store errors
// repo errors stores in repo package
// and can be returned from repo and used in service
// but they can't be used in handler

// errors that handler returns
// must be defined in service layer
// all repo errors should be returned by service errors in a service layer

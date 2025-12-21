package main

// >> case
// for example i have code that wants to use this interface
type Target interface {
	Operation()
}

// this is the struct that have metod AdaptedOperation()
// and i want to use this method instead of Operation()
// but i cant, because this struct do not implement Target interface
// ant if i cant to change this struct code(any reasons like SOLID)
// i must realise adapter
type SomeStructIWantToUseAsTargetInterface struct {
}

func (adaptee *SomeStructIWantToUseAsTargetInterface) AdaptedOperation() {
}

// to use my struct method as Target interface
// i create third struct
// place into link on my struct
// implements Target interface
// and rum my func into interface func
// then i pass this struct instead of Target interface
type Adapter struct {
	*SomeStructIWantToUseAsTargetInterface
}

func (adapter *Adapter) Operation() {
	adapter.AdaptedOperation()
}

//>> parallel test running
//<< when to use it 
// only to check instanses then must work in paralele

//<< implementation
// to run test in parralel
// i just need type in first line of test
func TestSomeFunc(t *testing.T){
    t.Parralel()
    // ...code of test func...
}
// then all tests in this function will run in paralele

//! if u use tests in paralel be scared closing of db in defer
//  because all tests runs in defferent gorutines
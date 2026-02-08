//>> logging
//<< Log Logf
// logs the messages through the test 
// logs shows only when fails or with verbose flag
t.Log("log message")
t.Logf("formatted message with some var: %s", someVar)

//>> fails
//<< Fail()
// fails the test and continue the test
t.Fail()

//<< FailNow()
// fails and exit
t.FaleNow()

//>>  Logging + Failing
// when you want to keep running use this two
//<< Error = Log + Fail
t.Error("err message")

//<< Errorf = Logf + Fatal
t.Errorf("formatted err with some var: %s", someVar)

// when you want to stop test use this two
//<< Fatal = Log + FailNow
t.Fatal("err message")

//<< Errorf = Logf + FatalNow
t.Fatalf("formatted err with some var: %s", someVar)
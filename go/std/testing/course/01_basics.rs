//>> base rules
// - write test only when it is needed
// - define goal of test, and do not forget it

//>> naming
//<< file naming
// - sorceFileToTestName_test.go - name of source file to test + _test.go

//<< test func naming
// TestFuncName(t.testing.T) - format of naming test function
// TestИмяСтруктуры_ИмяФункции(t.testing.T) - format of naming test functon if i test method of struct
// TestFuncName_caseNameFirstCharLowerCase(t.testing.T) or TestStructName_FuncName_caseNameFirstCharLowerCase(t.testing.T) - format of naming test functon when i test a specific test case

//<< vars naming
// got - returned result of tested funcion
// want - expected result
// args - args to pass to tested func

//<< message on Fail
// messages must be USEFULL!!!!
// i need to write is like this:
// "FuncName(%q) result = %q; want: %s", args, got, want
// for example if i checks the error as result, message will be like this
// "SendMessage(message) err= %q; want: %q", arg, got, want
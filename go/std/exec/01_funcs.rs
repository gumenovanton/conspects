//>> FOR WHAT 
// to run console Command 
// and read output 

//<< Command()
// create command by name and args
cmd := exec.Command("ls", "-la")

//<< Output() 
// get output from cmd 
out, err := cmd.Output()

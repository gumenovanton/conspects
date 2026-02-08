# IO REDIRECTION
// many of the commands write output to stdout in the terminal
// using the > operator, we can redirect the output to a file
// this command will overwrite the file if it already exists
date > date.txt

// using the >> operator, we can append the output to a file
date >> date.txt

# STANDART IO FLOWS
// < - stdin
// > or 1> - stdout
// 2> - stderr
// &> - stdout and stderr

## i can redirect all flows to a separate file
grep -r '^The' /etc/ 1>output.txt 2>errors.txt

## i can get rid of errors when do grep
// all errors will be redirected to /dev/null - blackhole of the linux system
grep -r '^The' /etc/ 2>/dev/null

## how to redirect stdout and stderr to a single file
grep -r '^The' /etc/ &>output.txt
// or
grep -r '^The' /etc/ >output.txt 2>&1
// or
grep -r '^The' /etc/ 1>output.txt 2>&1

#REDIRECTION FROM STDIN
// i can redirect stdin from the file to a program
// i can send an email from a file
sendemail someone@example.com < input.txt

## sort to end of file
sort <<EOF

## input string line
// in bash calculator
bc <<<1+2

# PIPING
// i can redirect output of one command to another command
// get all uncomment lines, sort them and output
grep -v '^#' /etc/login.defs | sort

## create columns in output
grep -v '^#' /etc/login.defs | sort | column -t

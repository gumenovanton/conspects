# PS
// info about processes in ASC order
// has two syntax BSD and Unix style

// get processes that are lounched on curent terminal window or session
$ ps

// get all processes with BSD syntax
// in [] - curnal processes
$ ps aux

// cpu - how mutch cpu use the process(100%-1 core)
// mem - how much memory use the process(% of all awailable)
// time - how much processor time it takes at all

// get info about process 1 by id in user oriented format
$ ps u 1

// get all processes by a user
$ ps u -U ang

// get all processes in my terminal and in my menage terminal TTY
// here i can get a niceness (see next)
$ ps l

// get all processes in long list format
$ ps lax

// get all processes tree
$ ps fax
$ ps faux

# TOP
// info about processes in CPU DESC order

// get all processes
$ top

# PGREP
// get processes by name
pgrep -a syslog

# NICENESS
// niceness
// - this is priority of process, between -20 and 19
// if two processes runs in a time, one with neceness -20, another with 19
// and if both needs 100% of CPU
// first takes all CPU awailabie
// second tekes just a little of CPU in downtime

// set niceness
$ nice -n 11 bash

// regular users can asign only values of niceness between 0 and 19
// to set lower i need to use sudo
$ sudo nice -n -5 bash

// change niceness of a running process by PID, should do it with sudo
$ sudo renice 7 5543

# SIGNALS
// this is the high priority messages that can be sent by OS to the process
// that meens hey process, stop doing your jub and do what i need
// and process can handle this signals
// exept sig.Stop and sig.Kill, process cant ignore them
// on sig.Stop the process should stop,
// if it not respond sig.Kill kills the process

// get list of signals
$ kill -L

// send a signal to the process with full syntax
& kill -SIGKILL 3232
// send a signal to the process with short syntax
& kill -KILL 3232
// send a signal to the process with number syntax
& kill -9 3232

// kill processes by name, all process that matches with the inputed name
& pkill -KILL bash


# SYGNAL

SIGHUP - cut the connection
SIGINT - interrupt CTRL+C
SIGQUIT - quit CTRL+\
SIGKILL - unconditional kill, CANT HANDLE IT
SIGTERM - gracefull shutdown
SIGSTOP - stop the process, or pause it, CANT HANDLE IT, stoped processes are in RAM but do not use CPU, and never continue after reboot
SIGTSTP - terminal stop CTRL+Z, this is gantle SIGSTOP
SIGCONT - continue the process, stoped with SIGSTOP

# STOPPING, FOREGROUNDING, BACKGROUNDING the processes
// to stop process fe when there is opened vim editor use key comand (suspend the process, i can run it again)
CTRL-Z

// to continue, send it in the foreground use fg
$ fg

// to run a program into background, use &
$ go run main.go &
// it displays the PID of a process and i can continue to work in the terminal

// to get all backgrounded processes use jobs
$ jobs
// it returns a list with comands in background like
[1]+ Running        go run main.go

// to return the app in foreground
$ fg 1
// or
$ fg

// return process that CTRL-Z
$ bg 1
// or
$ bg

# WHAT FILES AND DIRECTORIES ARE IN USE
// which files and directories in use of the process
$ lsof -p 8234

// which processes use the file or directory
$ sudo lsof /var/log/message

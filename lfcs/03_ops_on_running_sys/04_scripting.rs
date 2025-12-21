# SCRIPTING
// bash can execute scripts using the files with .sh extension

## how to use
- create a file with .sh extension
- make the file executable using chmod +x filename.sh
- run the script using ./filename.sh

# SYNTAX
// script starts with
#!/bin/bash
// that means the script will be executed by bash

# - it's a comment line

## help by commands
// help if

// after it i can type any commands like into a bash
// like this
date >> /tmp/script.log
cat /proc/version >> /tmp/script.log

# VARIABLES
// all strings
joe="Joe"
x=3
y=5

## how to use
echo "Holla $joe"

## arithmetic operations
// to perform arithmetic operations numbers need to be enclosed in double quotes
new=$(($x+$y))

## ENVIRONMENT VARIABLES
$BASH - path to bash
$BASH_VERSION - version of bash
$HOME - path to home directory
$HOSTNAME - pc name
$HOSTTYPE - processor architecture
$RANDOM - random number between 0 to 32767
$OSTYPE - operating system type
$PWD - current working directory
$UID - user id
$USER - username
$DIRSTACK - directory stack
$EDITOR - text editor by default
$EUID - effective UID. If you used the su program to execute commands as another user, this variable contains the UID of that user, while $UID contains the real ID, which is set only at login.
$FUNCNAME - name of the current function in the script
$GROUPS - user groups
$LC_CTYPE - locale category for character classification and conversion
$OLDPWD - previous working directory
$PATH - path
$PPID - parent process ID
$SECONDS - time script has been running (in seconds)
$# - number of arguments passed to the script
$* - all arguments passed to the script (output as a string)
$@ - same as previous, but parameters are output in a column
$! - PID of the last background process executed
$$ - PID of the script itself

# USER INPUT
echo -n "Как вас зовут" // -n means no newline
read name
echo "Привет $name"

# PARAMETERS
// i can pass parameters to the script
// $1 $2 $3 $4 $5 $6 $7 $8 $9
// all other $[n]

// for example calculate
echo "$(($1 $2 $3))"

// run
bash script.sh 5 + 6

# IF
// operations
// -lt - lower then
// -gt - greater then
// -le - lower or equal
// -ge - greater or equal
// -eq - equal

// all spaces are important
#!/bin/bash
echo "Введите возраст"
read age
if [[ $age -gt 50 ]] || [[ $age -eq 50 ]]; then
        echo "Да ты пердун старый"
    elif [[ $age -gt 10 ]] && [[ $age -lt 50 ]]; then
        echo "Да ты в самом рассвете сил"
    else
        echo "Да ты мальчик"
fi

// commands returns status
// 0 - success
// 1 - failure
// and i can test output of the command
if grep -q '5' /etc/default/grub; then
    echo "grub has timeout of 5 seconds"
else
    echo "grub does now have a timeout of 5 seconds"
fi

# CASE
#!/bin/bash
echo "Введите бренд"
read brand
case $brand in
    samsung)
        echo "samsung"
    nokia)
        echo "nokia"
    lg)
        echo "lg"
    *)
        echo "shit"
esac

# ARRAYS
// arrays can contain numbers and strings
#!/bin/bash

## array creation
Arr=(2 5 4)
Arr2=(3 string another 6)

## first element
echo ${Arr[0]}


## all elements
echo ${Arr2[@]}

## output indexes
echo ${!Arr[@]}
echo ${!Arr2[@]}

## accsess to element
echo ${Arr[2]}
echo ${Arr2[0]}

## length
echo ${#Arr[@]}
echo ${#Arr2[@]}

## length of an element
echo ${#Arr[1]}
echo ${#Arr2[2]}

## push
Arr[3]=fourth

## iteration
#!/bin/bash
Arr=(2 5 88 4)
for i in ${!Arr[@]}; do
    echo "${Arr[$i]}";   // здесь ковычки можно опустить, должно заканчиваться на ;
done

# CICLES
// for in
for i in 1 2 3; do
    echo $i;   // здесь должно заканчиваться на ;
done

// classic
for ((i=0; i<10; i++)); do
    echo $i;
done

// do while
n=1
while [ $n -lt 4 ]; do
    echo $n;
    n=$(( $n+1 ));
done

# FUNCS
list_files(){
    echo "Вывожу содержимое папки"
    cd Input
    ls;
}
list_files

## TEST
// help
$ help test
// test for example if file exists
// all flags look for help
if test -f "file.txt" ; then
    echo "File exists"
else
    echo "File does not exist"
fi

//or shorter
if [ -f "file.txt" ]; then
    echo "File exists"
else
    echo "File does not exist"
fi

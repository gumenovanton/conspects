## using an apropose command to filter results by categories
apropos -s 1,4 director
// it outputs all commands witch discription have "director" only in 1 and 4 category

## autocomplition
// if i type
systemc
// and press the Tab key
// it autocomplete it to
systemctl

// if i type
par
// and press the Tab key
// i don't get autocomplition
// because there is many commands that starts from "par"
// if i press a Tab key again
// i got output with all commands that starts with "par"

## autocomplition
// the autocomplition works with paths to
// type
ls /
// and press the Tab key
// and it outputs all foulders in the root
// then type
ls /h
// and it will autocompleat with
ls /home/

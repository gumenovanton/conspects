# SEARCH
// grep will output only lines with entered fragment

## case sensitive search
grep 'Arch' /etc/os-release

## ignore case
grep -i 'arch' /etc/os-release

## search in all files in a directory and in all nested directories
grep -r 'arch' /etc/

## invert mathc
// lines without a word
grep -v 'arch' /etc/os-release

## only words not the part of word
grep -w 'arch' /etc/os-release

## only matching without string context
grep -o 'arch' /etc/os-release

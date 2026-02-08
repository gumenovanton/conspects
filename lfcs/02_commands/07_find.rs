# SEARCHING FILES
## find by extention
find /usr/share/ -name '*.jpg'

## find by size
// find files which size less thes 10Mb
find /lib64/ -size +10M

## find by time of editing
// find files that chanded less then a minute ago
find /dev/ -mmin -1

# FIND PARAMS
## -name -iname
// search by name, case sensitive
find -name myfile
find -name 'o*' // all files that starts with a lower case o

// search by name, case unsensitive
find -iname myfile

## -mmin -mtime -cmin
// -mmin - search files by last modified param
// less then 5 minutes
find -mmin -5
// more than 5 minutes
find -mmin 5

// -mtime - search files by last modified param
find -mtime -2

// -cmin - search files by last changed param(metadata, like permissions)
find -cmin -5

## -size
// c - bytes
// k - kilobytes
// M - megabytes
// G - gigabytes

// exactly the same
find -size 512k
// lower than
find -size 512k
// greater than
find -size 512k

# AND, OR, NOT
// AND
find -name 'f*' -size 700M

// OR
find -name 'f*' -o -size 700M

// NOT
find -not name 'f*'
find \! -name 'f*'

# SEARCH BY PERMISSONS
// exactly the same
find -perm 664
find -perm u=rw,g=rw,o=r

// at least
find -perm -664
find -perm -u=rw,g=rw,o=r

// any of this
find -perm /664
find -perm /u=rw,g=rw,o=r

// by one permisson
find -perm 600
find -perm -100
find /! -perm -o=r

## WHAT A USER AND GROUP FOR FILES AND DIRS
-rwxrwxrwx aaron family ...
// aaron - is a user - he is an owner, only he and a root user can change file permissions
// family - is a group -

# how to change group
chgrp wheel my_file_or_directory

# how to list user groups
groups

## HOW TO CHANGE OWNER
// only the root user can change owner of a file
sudo chown jane my.txt

# how to change both user and group
sudo chown aaron:family my.txt

## PERMISSIONS
// < - > < rwx > < rwx > < rwx > - < type > < what can owner do > < what can group users do > < what can other users do >
// - - no permissions
// r - read a file or a directory
// w - write to a file or a directory
// x - execute a file or execute into directory

# hard link types
// - - file
// d - directory
// c - character device
// l - soft link
// s - socket file
// p - pipe
// b - block device

# how to change permissions
// chmod permissions my_file_or_directory

# how to add permissions
// FOR     OPTION  EXAMPLE
// user    u+      u+w / u+rw / u+rwx
// group   g+      g+w / g+rw / g+rwx
// others  o+      o+w / o+rw / o+rwx

// example
chmod u+w my.txt

# how to remove permissions
// FOR     OPTION  EXAMPLE
// user    u-      u-w / u-rw / u-rwx
// group   g-      g-w / g-rw / g-rwx
// others  o-      o-w / o-rw / o-rwx

// example
chmod u-w my.txt

# how to set exact permissions
// FOR     OPTION  EXAMPLE
// user    u=      u=w / u=rw / u=rwx
// group   g=      g=w / g=rw / g=rwx
// others  o=      o=w / o=rw / o=rwx

// example
chmod u=w my.txt

// set all empty
chmod g= my.txt
// its equal to
chmod g-rwx my.txt

# how to change permissons for all
chmod u+rw, g=r, o= my.txt

# haw to set permissions in octal format
// 0 - none
// 1 - execute
// 2 - write
// 4 - read

// 3 - execute and write
// 5 - execute and read
// 6 - read and write
// 7 - all

// r w - | r - - | - - -
// 1 1 0 | 1 0 0 | 0 0 0
//   6       4       0

// example
chmod 640 my.txt

## SUID BIT
-rwSrw-r--
// S - tells us that this file can be executed by any user exept owner
// and permissions was -rw-rw-r-- before suid was set
-rwsrw-rw--
// s - tells us that this file can be executed by any user and an owner
// and permissions was -rwxrw-r-- before suid was set

# how to set suid bit
// -rwSrw-r--
chmod 4664 my.txt
// -rwsrw-rw--
chmod 4764 my.txt

# how to unset suid bit
chmod u-s my.txt

# how to search them
find . -perm /4000

## SGID BIT
-rw-rwS-r--
// S - tells us that this file can be executed by any group
// and permissions was -rw-rw-r-- before sgid was set
-rw-rws-r--
// s - tells us that this file can be executed by any group
// and permissions was -rw-rwxr-- before sgid was set

# how to set sgid bit
// -rw-rwSr--
chmod 2664 my.txt
// -rw-rwSrw--
chmod 2764 my.txt

# how to unset sgid bit
chmod g-s my.txt

# how to search them
find . -perm /2000

## STICKY BIT
drwxrwxrwt
// t,T - tells us that files from this folder can be delited only by owner or root
// owner can set a sticky bit but can't remove it
// t - means than old permissions was drwxrwxrw-
// T - means than old permissions was drwxrwxrwx

# how to set sticky bit
chmod +t mydir
// or
chmod 1777 mydir

# how to unset sticky bit
chmod -t mydir

# how to search them
find . -perm /1000

## HOW TO SEARCH FILES WITH ALL BITS
// just use combination that i need
// for example
find . -perm/7000
find . -perm/5000
find . -perm/3000

# WHAT IS A SOFT LINK
// this is a file that points to a path
// like link in windows
// and if i move hardlink, a soft link will be broken
// unlike hard links i can create a soft link to a directory
// or a file and a directory in other disk

// creating a soft link
ln -s path_to_target_file path_to_link_file
ln -s /home/aaron/Pictures/family_dog.jpg family_dog_shortcut.jpg

## how to define i have a soft link or a hard link
ls -l
// will outputs permissons
// if
-rwxrwxrwx
// its a hard link
// else if
lrwxrwxrwx
// its a soft link

## how to output a soft link path
readlink family_dog_shortcut.jpg

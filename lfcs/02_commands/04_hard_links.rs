## what is a hard link
// when i type an ls command it outputs list of hard links (files)
// a hard link point to a node, and a node point to data in memory
// if i have two users and i want to share some file fore both
// a do not need to copy a file, it's very expensive
// because i can have 10 users in my unit, and store 10 units of equal files
// it's a bad idea
// the better approach is to copy a hard link
// and when one user delete a file, he deletes only a hardlink
// there will be nothing with a node and with its data
// only when all users delete this file, its node and data will be deleted

# CREATING A LINK
ln /home/aaron/Pictures/some_picture.jpg /home/jane/Pictures/some_pictire.jpg
//<< after that i need add all users that link to this file in one group
useradd -aG family aaron
useradd -aG family jane
// and change permissions on one hard link, it changes permissions for all hard links
// because permissions it's a node param
chmod 660 /home/aaron/Pictures/some_picture.jpg

## limitations
// i can hard link only to files, not directories
// and i can hard link to file only on one disk

## displaing a node number and a links count of a file
stat some_picture.jpg

# LIST DIRECTORIES
## list directories
ls

## list all directories (with hidden)
ls -a

## list long listing format (with info)
ls -l /var/log

## i can combine it
ls -a -l
// or
ls -al

## ls with human readable size format
ls -alh

## print working directory path
pwd

# CHANGE DIRECTORY
## change directory with absolute path
cd /home/ang/Go

## go up of directory tree
// one level up
cd ..
// two level up
cd ../..

## return to previous location
// if i type
cd /
// /home/ang/Go
// and i want to return to a /home/ang/Go i can type
cd -

## go to home
cd

# CREATE A FILE
touch filename.extention

# CREATE A DIRECTORY
// creates directory in working directory
mkdir name

## create a directory with all path directories if they do not exists
mkdir -p name

# COPY FILES
// destination better write with traling / - it's a good practice
cp source destination/

## copy and rename
cp my.txt destination/my_copy.txt

## copy directories with all files (recursive)
cp -r source/ destination/

## move files or directory
mv source_file destination/
// or
mv source_dir/ destination/

# MOVE WITH RENAMING A FILE OR A DIRECTORY
// renames a file in the current directory
mv my.txt my1.txt

// move and rename a file in an another directory
mv my.txt some_dir/my1.txt

## delete files
rm my.txt

## delete a directory
rm -r my_dir

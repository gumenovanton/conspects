# BACKUP FILES TO REMOTE SYSTEM
// here only a standart tools

## how it works
// using rsync command i syncro folder on my PC with remote system server
// ssh deamon must be running on remote server
rsync -a my_dir/ ang@9.9.9.9:/path/to/remote/folder/

// when you run this command in the second time
// this command will copy all the files that has changed
// and skip all data that up to date

## reverse command
// that command will create all the files in my_dir/
rsync -a ang@9.9.9.9:/path/to/remote/folder/ my_dir/

## syncronizing two local folders
// that command will syncronize two local folders
rsync -a my_dir/ my_dir2/


# IMAGING
// to create image of hole disk
sudo dd if=/dev/sda of=/path/to/image.raw bs=1M status=progress
// if=/dev/sda - path to disk
// of=/path/to/image.raw - path to image file
// bs=1M - block size, speed up the process
// status=progress - show progress of image creation

// to restore image of disk(never run it on virtual machine, because it will overwrite all data on disk)
// i just need to reverse if with of
sudo dd if=/path/to/image.raw of=/dev/sda bs=1M status=progress

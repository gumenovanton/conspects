# ARCHIVING FILES AND DIRECTORIES
## goals
// when i have a web site, for example,
// where i have a thousand files that i don't want to loose
// i go through three steps:
// 1. create a tar archive of the files
// 2. compress the tar archive using gzip
// 3. upload the encrypted archive to a secure location

#TAR
## what is tar?
tar - means tape archive
// tar is a packer and unpacker

## how to watch files in tar archive
// the longest command
tar --list --file archive.tar
// the shortest command
tar tf archive

## how to create a tar archive
// the longest command
tar --create --file archive.tar file1 file2 directory/
// the shortest command
tar cf archive.tar file1 file2 directory/

## to append files to a tar archive
// the longest command
tar --append --file archive.tar file4 file5 directory/
// the shortest command
tar rf archive.tar file4 file5 directory/

## how to extract files from a tar archive
// the longest command
tar --extract --file archive.tar
// the shortest command
tar xf archive.tar

## how to extract to another directory
// the longest command
tar --extract --file archive.tar --directory /path/to/destination
// the shortest command
tar xf archive.tar -C /path/to/destination

// tar although stores all permission and ownership information
// other people can't access the files and extract them
// but root can
sudo tar xf archive.tar -C /path/to/destination

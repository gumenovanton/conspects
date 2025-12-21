## COMPRESSION AND ARCHIVING BY ONCE
## with gzip
tar --create --gzip --file archive.tar.gz file1 file2 directory/
tar -czf archive.tar.gz file1 file2 directory/

## with bzip2
tar --create --bzip2 --file archive.tar.bz2 file1 file2 directory/
tar -cjf archive.tar.bz2 file1 file2 directory/

## with xz
tar --create --xz --file archive.tar.xz file1 file2 directory/
tar -cJf archive.tar.xz file1 file2 directory/

## the shortest command
// compression method will be determined by the file extension
tar caf archive.tar.xz file1 file2 directory/

## tar can understand how to unpack by self
tar --extract --file archive.tar.xz
tar xf archive.tar.xz file1

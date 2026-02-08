// compressed files not too useful
// but it useful to transform data from system to system

# TYPES OF COMPRESSORS
// gzip
// bzip2
// xz
// zip

## how to compress with them
// all of them compress files and delete the original file

// gzip gives file.gz
zip file1
// bzip2 gives file.bz2
bzip2 file2
// xz gives file.xz
xz file3
// zip gives file.zip
zip archive.zip file4
zip -r archive.zip directory/

## how to decompress with them
// all of them decompress files and delete the compressed file

// gzip gives file.gz
gunzip file1.gz
// bzip2 gives file.bz2
bunzip2 file2.bz2
// xz gives file.xz
unxz file3.xz
// zip gives file.zip
unzip archive.zip

## how to avoid deletion
//gzip
gzip -k file1
//bzip2
bzip2 -k file2
//xz
xz -k file3

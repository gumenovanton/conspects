# PRINT FILES IN CONSOLE
## print a file
cat /home/myfile

## print reversed file content
tac /home/myfile

## print a tail of file
// print last 10 lines
tail /home/myfile
// print last 20 lines
tail -n 20 /home/myfile

## print a head of file
// first 10 lines
head /home/myfile
// 20 first lines
head -n 20 /home/myfile

# REPLACE CONTENT IN FILE
// s - substitute
// g - globaly, it means in entire file
// find every canda and show a preview of replacing by a canada
sed 's/canda/canada/g' my.txt
// find only the first canda in line and show a preview of replacing by a canada
sed 's/canda/canada/g' my.txt

// to replase
sed -i 's/canda/canada/g' my.txt

# EXTRACT WORDS BY LINES
## cut
// extract but not delete all first words from every line with space delimiter
// nami cars sam
// bob john saimon
cut -d ' ' -f 1 my.txt
// nami
// bob

// extract second word frome every line and put in file
// nami,cars,sam
// bob,john,saimon
cut -d ',' -f 2 my.txt > words.txt
// cars
// john

## uniq
// eqtracts content from a file
// withowe 2 simmilar lines
// man
// wow
// man
// man
// wow
uniq my.txt
// man
// wow
// man
// wow

## sort
// sort the file
sort my.txt
// man
// man
// man
// wow
// wow

## to get uniq records i can
sort my.txt | uniq

# COMPARE FILES
// it needs to compare old and new configs

## diff
// diff shows only difference of two files
diff one two
// but results hard to read
// 3,4c3
// < man
// < man
// ---
// > manoo

// output with content
// ! - shows difference
diff -c one two
// *** one	2025-05-17 20:50:15.464761610 +0400
// --- two	2025-05-17 20:59:11.693684658 +0400
// ***************
// *** 1,5 ****
//   man
//   wow
// ! man
// ! man
//   wow
// --- 1,4 ----
//   man
//   wow
// ! manoo
//   wow

## side dy side difference
diff -y one two
// or
sdiff one two
// man								man
// wow								wow
// man							      |	manoo
// man							      <
// wow								wow

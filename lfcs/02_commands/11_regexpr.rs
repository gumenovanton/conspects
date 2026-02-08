# SEARCH WITH GREP AND REGEX
// i can use two types of regex in grep command
// default and extended
// just always use extended
grep -E '^cat$' myfile
// or just use egrep
egrep '^cat$' myfile

^ - start of the word operator
$ - end of the word operator
. - one character
\ - escape a character operator
* - a previous character zero or more times
+ - a previous character one or more times
? - a previous character is optional
{m,n} - a previous character  from n to m times                         // {,3} {3,} {3,8}
{n} - a previous character exact n times
[] - one character from a range or set of symbols                       // [a-z] [a-zA-z0-9] [aMBe]
[^] - one character exept characters from a range or set of symbols     // [^a-z] [^a-zA-z0-9] [^aMBe]
() - subexpressons

## logical operators
// works between two regular expressons
| - or      // 'enable|desable'

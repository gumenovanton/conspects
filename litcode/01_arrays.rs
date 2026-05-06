01. TWO SUMMS
// range through an array
// get el - target, and push it to hash with key of diff and val of element idx
// find in hash element with key equals to element, if exists
// return an array with value from hash and idx of current element

14. LONGEST COMMON PREFIX
// take first word as the prefix
// range through the array
// if !strings.HasNoPrefix(str[i], prefix)
// prefix = prefix[:len(prefix)-1]
// if prefix = "" return ""

26. REMOVE DUPLICATES FROM SORTED ARRAY
// save first element as value to compare as compNum
// get second element as second changePosition
// go through array
// if next not equals to compNum put it to saved position, position++
// compNum = next
// go to next
// return changePosition

27. REMOVE ELEMENT
// range through array
// if element != target
// push it to pos idx
// and pos++
// return pos

// to optimize i can go from start and from end at an iteration

FOR_WITH_GENERATED_VALUES
// 1..100 - exclusive range from 1 to 99
// 1..=100 - inclusive range from 1 to 100
for i in 1..=100 {
}

CONSUMING COLLECTION
// if i have an array of nums
let nums: [i32; 9] = [3, 7, 22, 2, 6, 76, 87, 8, 926];
// and iterate like this
for num in nums {
    // all nums from array will be COPIED to local num var
    // and after the end of for loop local num will be destoyed
}

// but if i have a heap type fe an array or vec of strings
let words = ["mad".to_string(), "bad".to_string()];
// and iterate like this
for word in words {
    // all words from array will be MOVED to local num var
    // and after the end of for loop local word will be destoyed
}

// to fix it i should use iteration like this
for word in words.iter() {
    // word is a ref
}

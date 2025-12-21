#### INNER JOINS
// default type of a conditional join
// containes only matches from two table (all after WHERE)

// get all where user.id mathes bookmarks.user_id
SELECT users.id, users.firstname, bookmarks.user_id, bookmarks.id FROM users JOIN bookmarks ON users.id = bookmarks.user_id LIMIT 10
// 345 Kylegh 345 435345
// 345 Kylegh 345 23423
// 345 Kylegh 345 234256
// 345 Kylegh 345 34452
// 345 Kylegh 345 8977978
// 345 Kylegh 345 8789
// 345 Kylegh 345 98
// 444 Waylon 444 870
// 444 Waylon 444 870
// it means that
// Kylegh has 7 bookmarks
// Waylon has 2 bookmarks

>>>> if JOINS equal
// i can use USING key word instead of users.id = bookmarks.user_id
SELECT users.user_id, bookmarks.id FROM users JOIN bookmarks ON USING(user_id) LIMIT 10

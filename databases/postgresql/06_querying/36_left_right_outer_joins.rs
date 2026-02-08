#### LEFT JOIN
// get every single user(from left table) even if he have no bookmark
// and get all from the bookmarks table if it exists for a user
SELECT * FROM users LEFT JOIN bookmarks ON users.id = bookmarks.users_id

#### RIGHT JOIN
// get every single bookmark(from right table) even if has no users
// and get all from the users if it exists for bookmark
SELECT * FROM users RIGHT JOIN bookmarks ON users.id = bookmarks.users_id

#### FULL JOIN
// get all from the left and from the right tables where user.id mathes bokmarks user.id
// plus all other from left table
// plus all other from right table
SELECT * FROM users FULL JOIN bookmarks ON users.id = bookmarks.users_id

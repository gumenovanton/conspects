#### SUBQUERIES
// joining field from the left table with the result of the subquery

// for example create index
CREATE INDEX bookmarks_secure_url ON bookmarks(user_id, (starts_with(url, 'https')));

// and use it in the query
// get first_name and url forom users where the value in index is true
SELECT firstname, url
FROM
    users INNER JOIN (
        SELECT * FROM bookmarks WHERE starts_with(url, 'https') IS true
    ) AS bookmarks_secure ON users.id = bookmark.user_id
LIMIT
    10;

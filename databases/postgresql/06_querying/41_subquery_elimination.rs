#### SUBQUERY ELIMINATION
// if i want to select all user data only for users that have bookbarks more then 16
select * from users where id in(
    select user_id from bookmarks
        group by user_id
        having count(*)>16
);
// i can replace in with exists, but never do it

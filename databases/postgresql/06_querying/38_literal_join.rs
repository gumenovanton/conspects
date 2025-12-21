#### LITERAL JOIN
// used very rare
// addition to all type joines
// means for every row do this subquery
// I DO NOT UNDERSTAND IT
SELECT
    users.id, firstname, url
FROM
    users LEFT JOIN LATERAL(
        // select one of all where user_id is the latest
        SELECT * FROM bookmarks WHERE user_id=users.id ORDER BY id DESC LIMIT 1
    ) AS most_resent_bookmark ON TRUE
LIMIT 10;

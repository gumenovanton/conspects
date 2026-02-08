#### TSVECTOR
// this is sorted vector of distinct leximes(basic unit of language)
// 'the' is not lexime
SELECT to_tsvector('the quick brown fox jumps over the laziness dog'); // no s, over, the
// 'brown':3 'dog':9 'fox':4, 'jump':5 'lazi':8,9 'quick':2

#### TSQUERY
// stores one lexime
SELECT to_tsquery('lazy');
// 'lazi'

>>>> search leximes in the text
SELECT to_tsvector('the quick brown fox jumps over the laziness dog') @@ to_tsquery('lazy'); // true

#### HOW TO OPERATE WITH TEXT TO SEARCH
CREATE TABLE ts_example(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    content text,
    search_vector_en TSVECTOR GENERATED ALWAYS AS (to_tsvector('english', content)) STORED
);

INSERT INTO ts_example (content) VALUES ('the quick borwn fox jumps over the dog');

// search
SELECT * from ts_example WHERE search_vector_en @@ to_tsquery('lazy');

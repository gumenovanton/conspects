# CHARS AND COLATIONS
// just for education
// UTF-8 - this is encoding
// en_US.UTF-8 - conation - the set of rules how to collate symbols with each other

## how to create your own collation
CREATE COLLATION en_us_ci (
    provider = icu, // international colation for unicode, just use it
    locale - 'en-US-u-ks-level1', // lets start from en-US, then unicode, with the least case sensitive level (forget about case checking)
    determenestic = false // some shit
)

// lets compare two strings with my rules
SELECT 'abc' = 'Abc' COLLATE "en_us_ci" AS result;

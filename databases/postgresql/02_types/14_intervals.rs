#### INTERVALS
// to store duration
// just duration
SELECT '1 year 3 months 3 days 04:05:06'::INTERVAL;

>>>> HOW TO SHOW INTERVAL STYLE
SHOW intervalstyle; // postgres

>>>> HOW TO CHANGE INTERVAL STYLE
SET intervalstyle = 'iso_8601';
// and then
SELECT '1 year 3 months 3 days 04:05:06'::INTERVAL; // P1Y3MO3D4H5M6S

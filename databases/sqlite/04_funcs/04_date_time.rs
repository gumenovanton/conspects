//>> DATE
// return date in "YYYY-MM-DD" format
date(time-value, modifier, modifier, ...)

//>> TIME
// calc time 
time(time-value, modifier, modifier, ...)

//>> DATETIME
// calc date & time
datetime(time-value, modifier, modifier, ...)

//>> JULIANDAY
// days from  (24 ноября 4714 до н.э.) in REAL type
julianday(time-value, modifier, modifier, ...)

//>> STRFTIME
// format date
strftime(format, time-value, modifier, modifier, ...)

//>> time-value
// without time-value returns current date and time
SELECT date() AS date, 
    time() AS time,
    datetime() AS datetime,
    julianday() AS julian;

// can take date and time in formats:
YYYY-MM-DD
YYYY-MM-DD HH:MM
YYYY-MM-DD HH:MM:SS
YYYY-MM-DD HH:MM:SS.SSS
YYYY-MM-DDTHH:MM (в качестве разделителя между датой и временем применяется разделитель - символ T в соответствии со стандартом ISO-8601)
YYYY-MM-DDTHH:MM:SS
YYYY-MM-DDTHH:MM:SS.SSS
HH:MM
HH:MM:SS
HH:MM:SS.SSS
now 
DDDDDDDDDD - days from julian in REAL or INTEGER

// set val
SELECT datetime('now');
SELECT datetime(1092941466);
SELECT date('2004-04-21') AS date,              -- YYYY-MM-DD
    time('16:45:21') AS time,                   -- HH:MM:SS
    datetime('2004-04-21T16:45') AS datetime,   --  YYYY-MM-DDTHH:MM 
    julianday('2004-04-21 16:45:21') AS julian; -- YYYY-MM-DD HH:MM:SS
    
SELECT date('now') AS date

//>> MODIFICATORS
// to change val of date and time

//<< NNN days
SELECT date('2021-12-31', '-7 days');   -- 2021-12-24

//<< NNN minutes
SELECT time('19:12', '35 minutes'); -- 19:47:00

//<< NNN.NNNN seconds
SELECT time('19:12', '-45.1 seconds');  -- 19:11:14

//<< NNN months
SELECT date('2021-12-31', '-7 months'); -- 2021-05-31

//<< NNN years
SELECT date('2021-12-31', '11 years');  -- 2032-12-31

//<< start of month
SELECT date('2021-11-26', 'start of month'');    -- 2021-11-01

//<< tart of year
SELECT date('2021-11-26', 'start of year''); -- 2021-01-01

//<< start of day
ELECT time('16:32', 'start of day'');   -- 00:00:00

//<< weekday N
SELECT date('2021-12-01', 'weekday 0''), -- 2021-12-05
        date('2021-12-01', 'weekday 1''), -- 2021-12-06
        date('2021-12-01', 'weekday 6''); -- 2021-12-04

//<< unixepoch
// returns seconds from UNIX epoch

//<< localtime
// returns localtime

//<< utc
// returns utc time
SELECT time('12:33:24', 'localtime'), -- 16:33:24
        time('16:33:24', 'utc'); -- 12:33:24

//>> strftime
//  format date
%d - день месяца в формате 00
%f - секунды в формате SS.SSS
%H - час в формате 00-24
%j - день года в формате 001-366
%J - количество дней с начала юлианской эпохи
%m - месяц в формате 01-12
%M - минута в формате 00-59
%s - количество секунда с 1970-01-01
%S - секунды в формате 00-59
%w - день недели в формате 0-6, где воскресение имеет номер 0
%W - номер недели года в формате 00-53
%Y - год в формате 0000-9999
%% - экраниурет символ %

SELECT strftime('%d.%m.%Y', '2021-12-01');  -- 01.12.2021

//>> STORE DATE AND TIME IN DB
// здесь нет специальных типов для хранения даты и времени
// их можно хранить в 
// TEXT как строки 2021-12-01 13:23:08
// REAL как дни с помощью julianday
// INTEGER как милисекунды с UNIX epoch

//<< DATES AND TIME AS TEXT
// sqlite can calculate
SELECT name, 
    date('now')-date_of_birth AS age 
FROM users;

// filter
SELECT name, date_of_birth 
FROM users
WHERE date_of_birth BETWEEN '1950-01-01' AND '1991-12-31';

// another one
SELECT * FROM orders
WHERE date_time BETWEEN datetime('now', 'start of day'') AND datetime('now');

// inserting in warious formats
INSERT INTO orders (date_time)
VALUES ('2021-11-30 22:01:15'),
(datetime('now')),
(datetime('now', '-5 hours'));

INSERT INTO users (name, date_of_birth) VALUES
('Tom', '1987-05-12'),
('Bob', date('now', '-41 years')),
('Sam', date('2021-11-29', '-25 years'));

//<< DATES AND TIME AS REAL
NSERT INTO orders (date_time)
VALUES (julianday('2021-11-30 22:01:15')),
(julianday('now')),
(julianday('now', '-5 hours'));

// sqlite can calculate
SELECT date_time, datetime(date_time) FROM orders;

//<< DATES AND TIME AS INTEGER
INSERT INTO orders (date_time)
VALUES (strftime('%s', '2021-11-30 22:01:15')),
(strftime('%s', 'now')),
(strftime('%s', 'now', '-5 hours'));

// sqlite can calculate
SELECT date_time, datetime(date_time, 'unixepoch') FROM orders;
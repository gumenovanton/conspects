//>> LENGTH
SELECT length('обороноспособность');	-- 18

//>> SUBSTR, SUBSTRING
// from index n elements
SELECT substr('обороноспособность', 8, 6),  -- способ
    substring('обороноспособность', 2, 3);  -- бор

// u can skip number of elements
SELECT substr('обороноспособность', 8), -- способность
    substring('обороноспособность', 2); -- бороноспособность

//>> LOWER UPPER
SELECT lower('Apple');  -- apple
SELECT upper('Apple');  -- APPLE

//>> INSTR
// returns index of first, indexes starts from 1
SELECT instr('обороноспособность', 'но');  -- 6

//>> RELACE
SELECT replace('iPhone 12 Pro', '12 Pro', '13');    -- iPhone 13

//>> LTRIM
// delete start spaces
SELECT ltrim('  Apple'); -- Apple

// cut from left and return remains
SELECT ltrim('iPhone 13 Pro', 'iPhone');  -- 13 Pro

//>> RTRIM
// delete start spaces
SELECT rtrim('Apple  '); -- Apple

// cut from left and return remains
SELECT rtrim('iPhone 13 Pro', 'Pro');  -- iPhone 13
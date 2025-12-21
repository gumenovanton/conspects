//>> ROUND
SELECT ROUND(1342.345);   	 -- 1342.0
SELECT ROUND(1342.345, 2);   -- 1342.35
SELECT ROUND(1342.344, 2);   -- 1342.34

//>> ABS 
SELECT abs(-123);	-- 123

//>> RANDOM
SELECT random(),	-- -1830670227460582245
	random();		-- -7041802194444300327

//>> MAX MIN
SELECT max(12, 10),	-- 12
	  max(1, 2, 5); -- 5

SELECT min(12, 10),	--  10
	min(1, 2, 5); 	-- 1
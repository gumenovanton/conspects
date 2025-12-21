//>> DELETE
DELETE FROM products
WHERE company='Huawei';

DELETE FROM products
WHERE company='Apple' AND price < 60000;

//<< DELETE ALL
DELETE FROM products;
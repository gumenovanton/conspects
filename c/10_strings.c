//>> СТРОКИ
// строки с с это массив символов

char message[] = "Hello";
printf("message: %s \n", message); // message: Hello
return 0;

//>> НУЛЕВОЙ СИМВОЛ
// каждая строка заканчивается нулевым символом
// то есть "Hello" имеет 6 символов а не пять

//>> STRING INTERNING .rodata
// так я положу строку в rodata и они будут статически вшиты в код
// и не изменяемые - это называется string Interning
char *str1 = "Hello World";
char *str2 = "Hello Board";
printf("str1 = %s \n", str1); // str1 = Hello World
printf("str2 = %s \n", str2); // str2 = Hello Board
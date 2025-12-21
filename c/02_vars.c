//>> переменные
// обьявление
int someVar;     // значение как карта ляжет
someVar = 5;     // проинициализировал
int nextVar = 6; // сразу проинициализировал

// обьявление нескольких штук
int number100 = 1, number200 = 2, number300 = 3;

//>> ТИПЫ ДАННЫХ
//<<  char,signet char
// - символ 8 ббит
//<<  unsigned char
// - символ 8 бит
//<<  short, short int, signed short и signed short int
// - число 16 бит
//<<  unsigned short, unsigned short int
// - число 16 бит
//<< int, signed int и signed
// от архитектуры либо 16 бит или 32 бита
//<< unsigned int, unsigned int и unsigned
// от архитектуры либо 16 бит или 32 бита
//<< long, long int, signed long int и signed long
// от архитектуры либо 32 бита или 64 бита
//<< unsigned long, unsigned long int
// от архитектуры либо 32 бита или 64 бита
//<< long long, long long int, signed long long int и signed long long.
// 64 бита
//<< unsigned long long, unsigned long long int
// 64 бита
//<< float
// 32 бита
//<< double
// 64 бита
//<< long double
// 80 бит
//<< void
// тип без значения

//<< ЗНАЧЕНИЯ ПО УМОЛЧАНИЮ
// все числа в зависимости от значения получают тип int, long int или long long int
// 16 32 или 64 бита соответственно

// если я хочу задать конкретный тип, нужно либо это явно указать
// либо воспользоваться суффиксом

//<< суффиксы
// u, U - unsigned int
// ul, Ul - unsigned long int
// ll, LL - long long int
// ull, ULL - unsigned long long int

unsigned short number1 = 1u;
unsigned short int number2 = 2u;
short number3 = 3;
short int number4 = -4;
signed short number5 = 5;
signed short int number6 = -6;

long number7 = -2147483648l;
long int number8 = -2147483648L;
signed long number9 = 2147483647l;
signed long int number10 = 2147483647L;

unsigned long number11 = 4294967295ul;
unsigned long int number12 = 4294967295UL;

long long number13 = -9223372036854775807ll;
long long int number14 = 9223372036854775807ll;
signed long long number15 = -9223372036854775807LL;
signed long long int number16 = 9223372036854775807LL;

unsigned long long number17 = 18446744073709551615ull;
unsigned long long int number18 = 18446744073709551615ULL;

//<< спецификаторы для вывода в консоль
// unsigned short hu
// long ld
// unsigned long lu
// long longlld
// unsigned long long llu

printf("number1 = %hu\n", number1);
printf("number2 = %hu\n", number2);
printf("number3 = %d\n", number3);
printf("number4 = %d\n", number4);
printf("number5 = %d\n", number5);
printf("number6 = %d\n", number6);
printf("number7 = %ld\n", number7);
printf("number8 = %ld\n", number8);
printf("number9 = %ld\n", number9);
printf("number10 = %ld\n", number10);
printf("number11 = %lu\n", number11);
printf("number12 = %lu\n", number12);
printf("number13 = %lld\n", number13);
printf("number14 = %lld\n", number14);
printf("number15 = %lld\n", number15);
printf("number16 = %lld\n", number16);
printf("number17 = %llu\n", number17);
printf("number18 = %llu\n", number18);
return 0;

//<< числа с плавающей точкой
double number = 3.14159;
printf("number = %f\n", number);
return 0;

//<< символы
char letter = 'A';
printf("letter: %d \n", letter); // letter: 65
printf("letter: %c \n", letter); // letter: A

//<< размер типа
int number = 2;
printf("sizeof(number) = %d \n", sizeof(number));

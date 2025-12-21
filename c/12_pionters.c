//>> УКАЗАТЕЛИ
// указатель также имеет тип
int x = 10; // определяем переменную
int *p;     // определяем указатель
p = &x;

// использование указателе при выводе в консоль
// tckb tcnm ghtleght;ltybt
// нужно явно указать что это указатель, сконвертить его
printf("%p \n", (void *)p);

//<< разименование
// можно достать переменную и присвоить ее другой переменной
int x = 10;
int *p = &x;
int y = *p;             // присваиваем переменной y значение по адресу из указателя p
printf("x = %d \n", y); // 10

//<< изменнеие значения по указателю
int x = 10;
int *p = &x;
*p = 45;
printf("x = %d \n", x); // 45

//>> ОБНУЛЕНИЕ УКАЗАТЕЛЯ
// NULL лежить в stdio.h
int *pa = NULL;

//>> УКАЗАТЕЛИ НА КОНСТАНТЫ
// для указателя на const нужно так же проставить const
const int a = 10;
const int *pa = &a;
printf("address=%p \t value=%d \n", pa, *pa);

//>> КОНСТАНТНЫЙ УКАЗАТЕЛЬ
// по нему можно менять значение но его адрес менять нельзя
int a = 10;
int *const pa = &a;
printf("value=%d \n", *pa); // 10
*pa = 22;                   // меняем значение
printf("value=%d \n", *pa); // 22

int b = 45;
// pa = &b;         так нельзя сделать

//>> КОНСТАНТНЫЙ УКАЗАТЕЛЬ НА КОНСТАНТУ
// тут и адрес и значение поменять нельзя
int a = 10;
const int *const pa = &a;

//*pa = 22;  так сделать нельзя

int b = 45;
// pa = &b;  так сделать нельзя

//>> УКАЗАТЕЛИТ И МАССИВЫ
// указатель на первый элемент - есть указатель на массив

// так получить можно первый элемент
int array[] = {1, 2, 3, 4, 5};
printf("array[0] = %d", *array); // array[0] = 1

// так второй
int array[] = {1, 2, 3, 4, 5};
int second = *(array + 1);       // получим второй элемент
printf("array[1] = %d", second); // array[1] = 2

// так можно перебрать массив
int array[5] = {1, 2, 3, 4, 5};

for (int i = 0; i < 5; i++)
{
    void *address = array + i; // получаем адрес i-го элемента массива
    int value = *(array + i);  // получаем значение i-го элемента массива
    printf("array[%d]: address=%p \t value=%d \n", i, address, value);
}

//<< ПЕРЕБОР МАССИВА С ПОМОЩЬЮ УКАЗАТЕЛЯ
int array[5] = {1, 2, 3, 4, 5};

for (int *ptr = array; ptr <= &array[4]; ptr++)
{
    printf("address=%p \t value=%d \n", (void *)ptr, *ptr);
}

//>> УКАЗАТЕЛЬ СО СТРОКОЙ
char *hello = "Hello METANIT.COM!"; // указатель на char - фактически строка
printf("%s", hello);

//>> .rodata
// так я положу строку в rodata и они будут статически вшиты в код
// и не изменяемые
char *str1 = "Hello World";
char *str2 = "Hello Board";
printf("str1 = %s \n", str1); // str1 = Hello World
printf("str2 = %s \n", str2); // str2 = Hello Board
//>> КОД ИЗ ВСНЕШНИХ ФАЙЛОВ
// если есть файл с переменной и жвумя функциями
#include <stdio.h>

char message[] = "Hello METANIT.COM";

void print()
{
    printf("Hello from extern function\n");
}
int sum(int x, int y)
{
    return x + y;
}

// и я хочу их использовать в другом файле
// для переменной мне нужно использовать слово extern
// а для функций определить прототипы
// в другом файле
#include <stdio.h>

extern char message[]; // переменная message из файла main.c, тут нельзя присваивать значения
void print(void);      // функция print из main.c
int sum(int, int);     // функция sum из main.c

int main(void)
{
    // выводим на консоль внешнюю переменную message
    printf("%s \n", message);
    // вызываем внешнюю функцию print
    print();
    // вызываем другую внешнюю функцию - sum
    printf("4 + 5 = %d \n", sum(5, 4));
    return 0;
}

//>> ПРИВАТНЫЕ ОБЬЕКТЫ
// если я не хочу чтобы какието переменные или функции
// были доступны извне
// я обьявляю их static
static char message[] = "Hello METANIT.COM";

static void print()
{
    printf("%s\n", message);
}
void hello()
{
    print();
}
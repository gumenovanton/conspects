//>> ПЕРВАЯ ПРОГРАММА
#include <stdio.h>                  // подключаем заголовочный файл stdio.h
int main(void)                      // определяем функцию main
{                                   // начало функции
    printf("Hello METANIT.COM!\n"); // выводим строку на консоль
    return 0;                       // выходим из функции
}

//<< компиляция и запуск
// gcc 01_hello_world.c - o holla &./ holla

//<< копиляция с параметрами
// скомпилировать код, говорю что по стандарту с17,
// смотреть не используемые переменные
// проверять строгое соответствие стандарту
// скомпилировать app.c в app и запустить
// gcc -std = c17 - Wall - pedantic app.c - o app &./ app

//>> для печати символов разных локалей нужно их обозначить
#include <stdio.h>
#include <locale.h>

int main(void)
{
    char *locale = setlocale(LC_ALL, "");

    printf("Привет мир! \n");
    return 0;
}
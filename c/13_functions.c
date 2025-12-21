//>> ФУНКЦИИ
//<< ОПРЕДЕЛЕНИЕ ФУНКЦИИ void
// определение функции должно быть до вызова
void hello()
{
    printf("Hello!\n");
}

int main(void)
{
    hello(); // вызов функции
    hello(); // вызов функции
    return 0;
}

//<< ПРОТОТИП
// если хочу определить потом
// нужно описать прототип до вызова

// описание прототипа
void hello(void);

int main(void)
{
    hello();
    hello();
    return 0;
}

// определение
void hello()
{
    printf("Hello!\n");
}

//>> ВХОДНЫЕ ПАРАМЕТРЫ
// при передаче по значению они копируются внутрь функции
// функция принимает массив символов
void print_message(char message[])
{
    printf("%s\n", message);
}

int main(void)
{
    print_message("Hello METANIT.COM!");
    print_message("Hello World!");
    print_message("Hello work...");
}

//>> ВОЗВРАЩАЕМОЕ ЗНАЧЕНИЕ
// вначале указываю возвращаемое значение
int sum(int x, int y)
{
    return x + y;
}

int main(void)
{
    int x = 25;
    int y = 15;
    int result = sum(x, y);
    printf("%d + %d = %d \n", x, y, result);
}

//>> ОБЛАСТЬ ВИДИМОСТИ
// переменные внутри функции видны только внутри нее
// и называются автоматическими, так как памяту удаляется
// внутри них
// если имя внешней переменной совпадает с внутренней,
// то внутренняя ее затеняет

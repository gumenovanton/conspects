//>> МАССИВЫ
// нобор значений одного типа с неизменяемой длинной
int numbers[4] = {1, 2, 3, 5};         // инициализация массива
printf("numbers[2] = %d", numbers[2]); // 3

// с сам может псчитать длинну
int numbers[] = {1, 2, 3, 5};

// частично
int numbers[5] = {10, 12}; // 10, 12, 0, 0, 0

// частично не по порядку
int numbers[5] = {[1] = 11, [3] = 13}; // 0 11 0 13 0

//>> РАЗМЕР
//<< в байтах
// размер возвращается типа size_t псевданим unsigned long long
int numbers[] = {5, 6, 7};
size_t size = sizeof(numbers);
printf("numbers size: %zu \n", size); // numbers size: 12
return 0;

//<< количество элементов
int numbers[] = {5, 6, 7};
size_t size = sizeof(numbers);
size_t count = sizeof(numbers) / sizeof(int);
printf("numbers size: %zu \n", size);   // numbers size: 12
printf("numbers count: %zu \n", count); // numbers count: 3

// или так
int numbers[] = {5, 6, 7};
size_t size = sizeof(numbers);
size_t count = sizeof(numbers) / sizeof(numbers[0]);
printf("numbers size: %zu \n", size);   // numbers size: 12
printf("numbers count: %zu \n", count); // numbers count: 3

//>> ПЕРЕБОР МАССИВА
int numbers[] = {10, 12, 13, 54, 43};
size_t count = sizeof(numbers) / sizeof(numbers[0]);
for (size_t i = 0; i < count; i++)
{
    printf("numbers[%zu] = %d \n", i, numbers[i]);
}

//>> КОНСАНТНЫЙ МАССИВ
const int numbers[3] = {11, 12, 13};
// numbers[1] = 22; // Нельзя изменить - массив константный
printf("numbers[1] = %d", numbers[1]); // numbers[1] = 22

//>> МНОГОМЕРНЫЕ
int numbers[3][2] = {{1, 2}, {4, 5}, {7, 8}};
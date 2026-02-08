//>> IF

int x = 60;

if (x > 60)
    printf("x is greater than 60 \n");
else if (x < 60)
    printf("x is less than 60 \n");
else
    printf("x is equal 60 \n");

// варианты более одной строки кладем в ковычки
int x = 60;

if (x > 60)
{
    printf("if statement \n");
    printf("x is greater than 60 \n");
}
else if (x < 60)
{
    printf("else if statement \n");
    printf("x is less than 60 \n");
}
else
{
    printf("else statement \n");
    printf("x is equal 60 \n");
}

//>> SWITCH
// break пишем по любому
switch (x)
{
case 1:
    printf("x = 1 \n");
    break;
case 2:
    printf("x = 2 \n");
    break;
case 3:
    printf("x = 3 \n");
    break;
default:
    printf("x is undefined \n");
    break;
}

//>> ТЕРАНРНЫЙ ОПЕРАТОР
int z = x > y ? x - y : x + y;
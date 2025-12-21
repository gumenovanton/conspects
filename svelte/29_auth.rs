//>> ПОТОК РЕГИСТРАЦИИ
// клиент кидает данные на эндпоинт регистрации или входа, получает токены в куки
// если все норм переадресация на целевую страницу
// на целевой странице делаются запросы данных, с отправкой кук, в функции Load
// куки парсятся на беке в мидлварях, если протухли возвращают ошибку,
// которая обрабатывается в, и если что не так переадресовка на авторизацию

//<< структура бека
// эндпоинт регистрации
// эндпоинт входа
// эндпоинт выхода
// эндпоинты API


//>> ПОДРОБНО
//<< ФРОНТЕНД РЕГИСТРАЦИЯ
// по кнопке после ввода пароля отправляем данные на сервак
const signUp = async () => {
    const response = await fetch('/api/create-user', {
        method: 'POST',
        credentials: "same-origin",
        body: JSON.stringify({
            email,
            username,
            password
        })
    });
    // не по goto, иначе куки не обновятся
    if (response.ok) {
        window.location.href = '/';
    }
};

//<< БЕКЕНД РЕГИСТРАЦИЯ
// получаю данные, если нет, отвечаю 400 "Invalid request"
// проверяю валидность, если нет отвечаю 400 "Bad request"
// ищу вбазе есть ли такие, если да отвечаю 405 "User already exists"
// создаю соль
// хеширую пароль с солью
// создаю user_id
// создаю refresh_token, как рандомный UUID
// сую пользователя в базу, если ошибка отвечаю 500 "Internal Server Error"
// генерирую JWT
// отвечаю 200
// и ставлю хедеры, для установи кук
return {
    status: 200,
    headers: {
        // import { dev } from '$app/env';
        // const secure = dev ? '' : ' Secure;';
        "set-cookie": [
            // expires in 90 days
            `refresh_token=${refresh_token}; Max-Age=${30 * 24 * 60 * 60}; Path=/; ${secure} HttpOnly`,
            `token=${token}; Max-Age=${15 * 60}; Path=/;${secure} HttpOnly`
        ]
    }
};

//<< ФРОНТЕНД ВХОД
// по кнопке после ввода пароля отправляем данные на сервак
const signUp = async () => {
    const response = await fetch('/api/login-user', {
        method: 'POST',
        credentials: "same-origin",
        body: JSON.stringify({
            username,
            password
        })
    });
    // не по goto, иначе куки не обновятся
    if (response.ok) {
        window.location.href = '/';
    }
};

//<< БЕКЕНД ВХОД
// получаю логин и пароль, если пусто, отвечаю 400 "Invalid request"
// проверяю валидность, если нет отвечаю 400 "Bad request"
// ищу в базе, если нет, отвечаю 401 "Wrong login or password"
// сравниваю пароли, если не правильный, отвечаю 401 "Wrong login or password"
// создаю refresh_token, как рандомный UUID
// генерирую JWT
// отвечаю 200
// и ставлю хедеры, для установи кук, новый рефрешь и новый аксесс токены
return {
    status: 200,
    headers: {
        "set-cookie": [
            `refresh_token=${refresh_token}; Max-Age=${refresh_token_expiresIn * 24 * 60 * 60}; Path=/; ${secure} HttpOnly`,
            `token=${token}; Max-Age=${15 * 60}; Path=/;${secure} HttpOnly`
        ]
    }
};

//<< АВТОРИЗАЦИЯ ФРОНТЕНД
// как на фронтенде понять что я авторизован
// в функции load страницы делаю запрос на бекенд аутентификации
// если ок то подгружаются данные
// если нет, редирект на вход
export const load: Load = async (input) => {
    const response = await input.fetch('/api/auth');
    const user = (await response.json()) as Session;
    if (!user.user_id) {
        // user doesn't exist
        return {
            status: 302,
            redirect: '/signin'
        };
    }
    return {
        props: {
            user
        }
    };
};

//<< АУТЕНТИФИКАЦИЯ БЕКЕНД
// в мидлвари парсим куки
// если аксесс токен не истек, толкаю в хендлер, возвращаю данные
// если истек, проверяю еслть ли рефреш токен, если нет сразу возвращаю 401 "Unauthorized user"
// и удаляю куку с рефреш токеном на фронте
// если refresh актуальный, генерю новый аксесс токен и кидаю на фронт

//<< ВЫХОД
// иду на эндпоинт singout
// и просто отвечаю обнулением токенов или удаляю их
return {
    status: 200,
        headers: {
            "set-cookie": [
                `refresh_token=; Max-Age=0; Path=/; ${secure} HttpOnly`,
                `token=; Max-Age=0; Path=/;${secure} HttpOnly`
            ]
        }
    };

//<< ОТЗЫВ ДОСТУПА
// нужно просто изменить refresh token в базе
Option
1. Проверка состояния (Boolean)
• is_some(): true, если есть значение.
• is_none(): true, если значения нет.
• is_some_and(f): true, если есть значение и оно подходит под условие f.
2. Работа со ссылками (References)
• as_ref(): Из &Option<T> делает Option<&T>.
• as_mut(): Из &mut Option<T> делает Option<&mut T>.
• as_deref(): Из Option<T> делает Option<&T::Target> (например, Option<String> -> Option<&str>).
• as_deref_mut(): Аналогично для мутабельных ссылок.
3. Извлечение значения (Unwrapping)
• unwrap(): Значение или паника.
• expect(msg): Значение или паника с вашим сообщением.
• unwrap_or(default): Значение или default.
• unwrap_or_else(f): Значение или результат функции f.
• unwrap_or_default(): Значение или дефолт типа (если реализован Default).
• unchecked_unwrap(): (только в nightly) Извлечение без проверки (небезопасно).
4. Трансформация и фильтрация (Mapping/Filtering)
• map(f): Превращает Option<T> в Option<U> через функцию f.
• map_or(default, f): Применяет f или возвращает default.
• map_or_else(default_f, f): Применяет f или вычисляет дефолт через default_f.
• filter(p): Оставляет Some, если предикат p истинен, иначе возвращает None.
• flatten(): Раскрывает вложенность: Option<Option<T>> -> Option<T>.
• and_then(f): (FlatMap) Если Some, вызывает f (которая сама возвращает Option).
5. Логические операции над двумя Option
• and(optb): Возвращает optb, если оба Some.
• or(optb): Возвращает первый Some, который встретит.
• xor(optb): Возвращает Some, только если ровно один из них Some.
6. Изменение на месте (In-place Operations)
• insert(value): Вставляет значение и возвращает ссылку на него.
• get_or_insert(value): Если там None, вставляет значение.
• get_or_insert_with(f): Если там None, вставляет результат функции.
• take(): Забирает значение, оставляя None на его месте.
• replace(value): Заменяет значение и возвращает старое.
7. Работа с побочными эффектами (Inspection) — Новое!
• inspect(f): Если Some, выполняет функцию f (например, лог) и возвращает исходный Option дальше.
• inspect_none(f): Если None, выполняет функцию f (идеально для ваших логов).
8. Конвертация типов
• ok_or(err): Option<T> -> Result<T, E>.
• ok_or_else(err_f): То же самое, но ошибка вычисляется лениво.
• unzip(): Из Option<(T, U)> делает (Option<T>, Option<U>).
• transpose(): Из Option<Result<T, E>> делает Result<Option<T>, E>.
9. Итераторы
• iter(): Возвращает итератор по значению (0 или 1 элемент).
• iter_mut(): Мутабельный итератор.
• into_iter(): Превращает в итератор (потребляет владение).
Совет: Если вы пишете код для продакшена, чаще всего вы будете использовать as_ref(), map(), and_then()
и unwrap_or_else().
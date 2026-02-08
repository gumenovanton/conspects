//>> КОНТЕКСТ
// контекст - это общий стейт для родителя и потомков
// он не глобальный
// нужен чтобы нет пробрасывать переменные через несколько компонентов
// контекст так же будет доступен в slot

//<< установка значения в одном компоненте
import { setContext } from 'svelte';

// просто добавляю значение по ключу
setContext('name', 'john');


//<< получение значения в другом и проверка на наличие
import { getContext, hasContext } from 'svelte';

let name;

if (hasContext('name')){
    name=getContext('name');
}

//>> РЕАКТИВНОСТЬ
// по умолчанию переменные контекста не реактивны
// если хочешь сделать реактивным, передавай $state

//<< РОДИТЕЛЬ
<script>
	import { setContext } from 'svelte';

	let value = $state({ count: 0 });
	setContext('counter', value);
</script>

<button onclick={() => value.count++}>increment</button>

//<< ПОТОМОК
<script>
	import { getContext } from 'svelte';

	const value = getContext('counter');
</script>

<p>Count is {value.count}</p>
# РЕАКТИВНОСТЬ
// реактивное состояние задается с помощью руны $state
<script>
	let count = $state(0);
</script>

<button onclick={() => count++}>
	clicks: {count}
</button>

# ГЛУБОКАЯ РЕАКТИВНОСТЬ МАССИВОВ И ОБЬЕКТОВ
// масиивы и обьекты будут реактивными при изменении
// даже если менять их один элемент или свойство
let todos = $state([
	{
		done: false,
		text: "add more todos"
	}
])
//! ЭТО ВАЖНО
// в todos - будет лежать прокси, типа копия обьекта,
// мутирование которого ни как не скажется на оригинале

# ЗАПРЕТ ЗЛУБОКОЙ РЕАКТИВНОСТИ
// запрещаю ререндеринг при изменении свойств
let person = $state.raw({
	name: 'Heraclitus',
	age: 49
});

// не будет ререндера
person.age += 1;

// а здесь будет ререндер
person = {
	name: 'Heraclitus',
	age: 50
};



# СНЕПШОТ
// если мне нужно передать реактивное состояние во внешнее апи или библиотеку
// мне нужно достать из прокси переменную
<script>
	let counter = $state({ count: 0 });

	function onclick() {
		// выведет `{ count: ... }` вместо `Proxy { ... }`
		// так я могу посылать это значение куда мне нужно
		console.log($state.snapshot(counter));
	}
</script>

# ГЛОБАЛЬНЫЙ СТЕЙТ
// нужно создать файл с именем shared.svelte.js
export const counter = $state({
	count: 0
});

// импортнуть в него глобальный стейт
<script lang="ts">
	import { counter } from './shared.svelte.js';
</script>

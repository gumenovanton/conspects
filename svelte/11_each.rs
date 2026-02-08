# ПЕЕРБОР МАССИВОВ
<script>
	const colors = ['red', 'orange', 'yellow', 'green', 'blue', 'indigo', 'violet'];
	let selected = colors[0];
</script>

<h1 style="color: {selected}">Pick a colour</h1>

<div>
	{#each colors as color, i}
		<button
			style="background: {color}"
			on:click={() => selected = color}
		>{i + 1}</button>
	{/each}
</div>

# EACH ELSE
{#each todos as todo}
	<p>{todo.text}</p>
{:else}
	<p>No tasks today!</p>// если массив пустой
{/each}

# УСТАКНОВКА КЛЮЧЕЙ
// если нужно добавлять или удалять какие то элементы
// для того чтобы svelte понял с каким элементом работаем
// нужно проставить ключи

<script>
	import Thing from './Thing.svelte';

	let things = [
		{ id: 1, name: 'apple' },
		{ id: 2, name: 'banana' },
		{ id: 3, name: 'carrot' },
		{ id: 4, name: 'doughnut' },
		{ id: 5, name: 'egg' }
	];

	function handleClick() {
		things = things.slice(1);
	}
</script>

<button on:click={handleClick}>
	Remove first thing
</button>

// здесь проставляю ключи
{#each things as thing (thing.id)}
	<Thing name={thing.name} />
{/each}

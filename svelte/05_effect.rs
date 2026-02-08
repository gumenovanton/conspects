# СОБЫТИЕ НА ИЗМЕНЕНИЕ РЕАКТИВНОГО СОСТОЯНИЯ
// я могу повесить обработчик на смену состояния с помощью руны $effect
// повесить можно на $state, $derived, $props
// работает только после загрузки в DOM
<script>
	let count = $state(0);

    // при смене count вызовется эта анонимная функция
	$effect(() => {
		if (count >= 10) {
			alert(`count is dangerously high!`);
			count = 9;
		}
	});

	function handleClick() {
		count += 1;
	}
</script>

// все асинхронные значения, типа после await и setTimeout
// не будут триггерить $effect

# ЗАПУСК СОБЫТИЯ ДО ОБНОВЛЕНИЯ DOM
// используй $effect.pre

// здесь я делаю автоскрол к компоненту перед обновлением DOM
<script>
	import { tick } from 'svelte';

	let div = $state();
	let messages = $state([]);

	// ...

	$effect.pre(() => {
		if (!div) return; // если не смонтирован

		// reference `messages` array length so that this code re-runs whenever it changes
		messages.length;

		// автоскрол при добавлении сообщения
		if (div.offsetHeight + div.scrollTop > div.scrollHeight - 20) {
			tick().then(() => {
				div.scrollTo(0, div.scrollHeight);
			});
		}
	});
</script>

<div bind:this={div}>
	{#each messages as message}
		<p>{message}</p>
	{/each}
</div>

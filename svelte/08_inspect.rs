# ИНСПЕКЦИЯ
// если состояние меняется
// я могу ловить его изменение и выводить в консоль
// с помощью руны $inspect
<script>
	let count = $state(0);
	let message = $state('hello');

    // выведет в консоль когда значения count или message изменятся
	$inspect(count, message);
</script>

<button onclick={() => count++}>Increment</button>
<input bind:value={message} />

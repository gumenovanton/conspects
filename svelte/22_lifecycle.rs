## onMount
// срабатывает когда компонент смонтирован на старницу
<script>
	import { onMount } from 'svelte';
    onMount(() => {
		console.log("mount")
	});
</script>

## beforeUpdate afterUpdate
// до и после обновления
<script>
	import {
		beforeUpdate,
		afterUpdate
	} from 'svelte';

	beforeUpdate(() => {
		// determine whether we should auto-scroll
		// once the DOM is updated...
	});

	afterUpdate(() => {
		// ...the DOM is now in sync with the data
	});

</script>

## tick
// можно вызвать когда угодно
// запускает пееррисовку Dome сразу при вызове
// смотри tutorial svelte

## onDestroy
// перед удалением компонента
<script>
	import { onDestroy } from 'svelte';
	import { count } from './stores.js';

	let count_value;

    // подписываюсь, одновременно получаю колбек чтоб отписаться
	const unsubscribe = count.subscribe(value => {
		count_value = value;
	});

    // отписываюсь перед удалением копонента
	onDestroy(unsubscribe);
</script>

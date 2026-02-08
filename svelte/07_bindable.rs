# ПРИВЯЗКИ
// можно сделать при помощи руны $bindable

// стейт обьявленный в родителе
<script>
	import FancyInput from './FancyInput.svelte';

	let message = $state('hello');
</script>

<FancyInput bind:value={message} />
<p>{message}</p>

// меняется при изменении в потомке
<script>
	let { value = $bindable(), ...props } = $props();
</script>

<input bind:value={value} {...props} />

//<< значение по умолчанию
let { value = $bindable('fallback'), ...props } = $props();

//>> contenteditable
// я могу сделать div полем ввода
// и могу вводить в него значения
// для элемента contenteditable
// доступны innerHTML и textContent

<script>
	let html = '<p>Write some text!</p>';
</script>

<div bind:innerHTML={html} contenteditable></div>

<pre>{html}</pre>

<style>
	[contenteditable] {
		padding: 0.5em;
		border: 1px solid #eee;
		border-radius: 4px;
	}
</style>

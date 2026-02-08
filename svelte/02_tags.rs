# ТЕГИ

## @html
// когда нужно рендерить html напрямую
<script>
	let string = `here's some <strong>HTML!!!</strong>`;
</script>

<p>{@html string}</p>

## @render
// рендерить сниппет
{#snippet sum(a, b)}
	<p>{a} + {b} = {a + b}</p>
{/snippet}

// отрендерить три раза
{@render sum(1, 2)}
{@render sum(3, 4)}
{@render sum(5, 6)}

// можно рендерить в зависимости от переменной
{@render (cool ? coolSnippet : lameSnippet)()}

## @const
// позволяет определить локальную переменную в шаблоне
{#each boxes as box}
	{@const area = box.width * box.height}
	{box.width} * {box.height} = {area}
{/each}

## @debug
// при изменении значения выводит его в консоль
// и приостанавливает выполнение если открыт dev tools
{@debug user}

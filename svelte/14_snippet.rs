# ВЫДЕЛЕНИЕ ШАБЛОНА В СНИППЕТ
// полезно когда один и тот же шаблон
// рендериться по условию или в each
## синтаксис
{#snippet name(param1, param2, paramN)}...{/snippet}

## пример
{#snippet sum(a, b)}
	<p>{a} + {b} = {a + b}</p>
{/snippet}

// отрендерить три раза
{@render sum(1, 2)}
{@render sum(3, 4)}
{@render sum(5, 6)}

//! они видны только в этом файле где определены

## передача в пропсах
// так как это функция ее можно передать в пропсах
<FilteredList
	data={colors}
	field="name"
	{header}
	{row}
></FilteredList>

// получить в переменные
<script lang="ts">
	let { data, field, header, row } = $props();

	// ...
</script>

// и рендерить в коде
<div class="header">
	{@render header()}
</div>

<div class="content">
	{#each filtered as d}
		{@render row(d)}
	{/each}
</div>

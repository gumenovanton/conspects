# ОЖИДЕНИЕ
// когда нужно например поставить спинер
{#await promise}
	<p>...waiting</p>
{:then number}
	<p>The number is {number}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

// можно написать и так если знаеш что промис не реджектнится
{#await promise then number}
	<p>The number is {number}</p>
{/await}

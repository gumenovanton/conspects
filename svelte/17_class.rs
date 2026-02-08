# СИНТАКСИС УСТАНОВКИ КЛАССА
// класс элемента можно тоглить так
<div class={isCool ? 'cool' : ''}>...</div>

// но можно и через синтаксис svelte
<div class:cool={isCool}>...</div>

// тек можно задать один или несколько классов
<div class:cool class:lame={!cool} class:potato>...</div>

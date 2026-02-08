# СИНТАКСИС ШАБЛОНА КОМПОНЕНТА
# ТЕГИ
// можно применять все HTML теги
<div>
	<Widget />// это компонент
</div>

# АТРИБУТЫ
// можно использовать все стандартные атрибуты
<div class="foo">
	<button disabled>can't touch this</button>
</div>

// и не обязательно писать в ковычках
<input type=checkbox />

// можно вставлять js выражения в ковычках
<a href="page/{p}">page {p}</a>

// булевые атрибуты исколючаются если = false
// и вставляются если true
<button disabled={!clickable}>...</button>

// когда имя атрибута и переменная совпадают, имя можно опустить
<button {disabled}>...</button>

# ПРОПСЫ
<button {disabled}>...</button>
<Widget {...things} />

# СОБЫТИЯ
<button onclick={() => console.log('clicked')}>click me</button>

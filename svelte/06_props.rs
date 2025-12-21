# PROPS
// отправляю в компонент так
<script>
	import Nested from './Nested.svelte';
</script>

<Nested answer={42} />

// принимаю так
<script>
	let props = $props();
</script>

<p>this component is {props.answer}</p>

// могу деструктурировать
<script>
	let { answer } = $props();
</script>

<p>this component is {answer}</p>

// если пропс не передан
// можно указать значение по умолчанию
let { adjective = 'happy' } = $props();

# ДЕСТРУКТУРИЗАЦИЯ С ОТДЕЛЕНИЕМ КОНКРЕТНЫХ ОТ ОСТАЛЬНЫХ
let { a, b, c, ...others } = $props();

# ОБНОВЛЕНИЕ В ДОЧЕРНИХ КОМПОНЕНТАХ
// стейт обьявленный в родителе и переданный в потомок
// при изменении в потомке, обновиться в родителе
// даже если это обьект
// но при изменении в родителе, в потомке не изменится
// и это плохо
//! не нужно менять пропсы без &bindable

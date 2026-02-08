//>> ПЛАВНОЕ ИЗМЕНЕНИЕ ЗНАЧЕНИЙ
// обычно изменение значений происходит за мнгновение
// но можно сделать это изменнеие плавным, для анимаций например

//>> TWEENED

<script>
	import { tweened } from "svelte/motion";
	import { cubicOut } from "svelte/easing";

    // задаю метод и длительность анимации
	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
        // можно задать также delay
        // и interpolate - функция кастомного изменения значения
        // все эти параметры так же можно передать в set и update
	});
</script>

// подписываюст
<progress value={$progress}></progress>

<button on:click={() => progress.set(0)}>
	0%
</button>

<button on:click={() => progress.set(0.25)}>
	25%
</button>

<button on:click={() => progress.set(0.5)}>
	50%
</button>

<button on:click={() => progress.set(0.75)}>
	75%
</button>

<button on:click={() => progress.set(1)}>
	100%
</button>

<style>
	progress {
		display: block;
		width: 100%;
	}
</style>

//>> SPRINGS
// это тяжело описать словами
// можно привязать обьект как хвостик за мышкой
// смотри туториал svelte
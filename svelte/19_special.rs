## <svelte:window>
// я могу повесить обработчик на событие на window
<svelte:window onkeydown={handleKeydown} />

// и могу привязаться к следующим свойствам window
// innerWidth
// innerHeight
// outerWidth
// outerHeight
// scrollX
// scrollY
// online — алиас для window.navigator.onLine
// devicePixelRatio

<svelte:window bind:scrollY={y} />

## <svelte:document>
// так же могу повесить обработчик на событие на document
// activeElement
// fullscreenElement
// pointerLockElement
// visibilityState

<svelte:document onvisibilitychange={handleVisibilityChange} use:someAction />

## <svelte:body>
// могу повесить хендлер на событие body
<svelte:body onmouseenter={handleMouseenter} onmouseleave={handleMouseleave} use:someAction />

## <svelte:head>
// могу править метаданные страницы
<svelte:head>
	<title>Hello world!</title>
	<meta name="description" content="This is where the description goes for SEO" />
</svelte:head>

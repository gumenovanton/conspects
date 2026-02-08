# BIND
// меняет стейт при изменени и значения
// в полях ввода
// в чекбоксах
// в группах
// и тп
// и наоборот

## <input bind:value>
<script>
	let message = $state('hello');
</script>

<input bind:value={message} />
<p>{message}</p>

## <input bind:checked>
<label>
	<input type="checkbox" bind:checked={accepted} />
	Accept terms and conditions
</label>

## <input bind:group>
<script>
	let tortilla = 'Plain';

	/** @type {Array<string>} */
	let fillings = [];
</script>

<!-- grouped radio inputs are mutually exclusive -->
<input type="radio" bind:group={tortilla} value="Plain" />
<input type="radio" bind:group={tortilla} value="Whole wheat" />
<input type="radio" bind:group={tortilla} value="Spinach" />

<!-- grouped checkbox inputs populate an array -->
<input type="checkbox" bind:group={fillings} value="Rice" />
<input type="checkbox" bind:group={fillings} value="Beans" />
<input type="checkbox" bind:group={fillings} value="Cheese" />
<input type="checkbox" bind:group={fillings} value="Guac (extra)" />

## <input bind:files>
<script>
	let files = $state();

	function clear() {
		files = new DataTransfer().files; // null or undefined does not work
	}
</script>

<label for="avatar">Upload a picture:</label>
<input accept="image/png, image/jpeg" bind:files id="avatar" name="avatar" type="file" />
<button onclick={clear}>clear</button>

## <select bind:value>
<select bind:value={selected}>
	<option value={a}>a</option>
	<option value={b}>b</option>
	<option value={c}>c</option>
</select>

<select multiple bind:value={fillings}>
	<option value="Rice">Rice</option>
	<option value="Beans">Beans</option>
	<option value="Cheese">Cheese</option>
	<option value="Guac (extra)">Guac (extra)</option>
</select>

// если value совподает с контентом value можно опустить
select multiple bind:value={fillings}>
	<option>Rice</option>
	<option>Beans</option>
	<option>Cheese</option>
	<option>Guac (extra)</option>
</select>

## <audio>
// можно забайнтить
// двунаправленно
currentTime
playbackRate
paused
volume
muted
// однонаправленно

duration
buffered
paused
seekable
seeking
ended
readyState

## <video>
// то же самое что audio плюс только чтение
videoWidth
videoHeight

## <img>
// только чтение
naturalWidth
naturalHeight

## <details bind:open>
<details bind:open={isOpen}>
	<summary>How do you comfort a JavaScript bug?</summary>
	<p>You console it.</p>
</details>

# ПРИВЯЗКИ К КОНТЕНТУ ТЕГА
// можно привязаться к контенту тега
innerHTML
innerText
textContent
<div contenteditable="true" bind:innerHTML={html} />

# ПРИВЯЗКА К РАЗМЕРАМ
// можно привязаться к следующим
clientWidth
clientHeight
offsetWidth
offsetHeight
<div bind:offsetWidth={width} bind:offsetHeight={height}>
	<Chart {width} {height} />
</div>

# ПРИВЯЗКА К ЭЛЕМЕНТО DOM
<canvas bind:this={canvas} />

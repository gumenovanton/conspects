# УСТАНОВКА И СОЗДАНИЕ ПРОЕКТА

## стравлю Ноду
// paru -S nodejs
// paru -S npm

## создаю проект svelteKit
// npx sv create myappname
// cd myappname
// npm run dev -- --open

## создание проекта на чистом svelte
npm create vite@latest

# РАБОТА С GO CO СТАТИКОЙ
// это канает только для статических страниц
// суть следующая
// я пишу сервер, отдающий статику, с любым API
// пишу фронт, и генерю статику в папку сервера

// для этого мен нужно установить генератор статики

## устанавливаю adapter-static
npm i -D @sveltejs/adapter-static

## настройка svelte.config.js
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: '../static',
			assets: '../static',
		})
	}
};

export default config;

## в каждую страницу добавляю пререндер
// для того чтобы сгенерировать статический файл
// создаю для каждого роута +page.js
// и в него записываю
export const prerender = true;

## разработка и генерация статики
// для того чтобы сгенерировать статику
npm run build

// для разработкеи в браузере
npm run dev -- --open


# НАСТРОЙКА TAILWIND чисто в SVELTE KIT
// https://tailwindcss.com/docs/guides/sveltekit

## ставлю
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

## правлю svelte.config.js
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: '../static',
			assets: '../static',
		})
	},
	preprocess: vitePreprocess()
};

export default config;

## правлю tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
	  extend: {}
	},
	plugins: []
  };


## ./src/app.css
// добавляю файл в корень
@tailwind base;
@tailwind components;
@tailwind utilities;

## добавляю стили в +layout.svelte
<script>
  import "../app.css";
</script>

<slot />

## просто работаю
npm run dev

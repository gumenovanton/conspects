//>> ПСЕВДАНИМЫ ПУТЕЙ
//<< $lib
// это есть главный псевданим указывающий на папку lib
// например компонент можно подтягивать так
import Setter from '$lib/components/molecular/Setter.svelte';

// но я могу создать свой псевданим


//<< создание своего псевданима
// иду в svelte.config.json
// и добавляю свой алиас в параметр alias

import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: '../static',
			assets: '../static',
		}),
        alias:{
            $utils:'./src/utils',
        }
	}
};

export default config;
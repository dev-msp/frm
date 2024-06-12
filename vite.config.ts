import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:3030',
				secure: false,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});

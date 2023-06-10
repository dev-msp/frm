import { browser, dev } from '$app/environment';
import { beforeUpdate, getContext, onMount, setContext } from 'svelte';

export const safeWindow = browser ? window : undefined;

const DEBUG = Symbol();

export const debug = () => {
	if (!dev) return;
	setContext(DEBUG, true);
};

export const getDebug = () => {
	if (!dev) return false;
	return getContext<boolean>(DEBUG) ?? false;
};

export const onMountDebug = (fn: () => unknown): void => {
	const isDebug = getDebug();
	onMount(() => {
		if (isDebug) {
			return fn();
		}
	});
};

export const beforeUpdateDebug = (fn: () => unknown): void => {
	const isDebug = getDebug();
	beforeUpdate(() => {
		if (isDebug) {
			return fn();
		}
	});
};

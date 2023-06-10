import isEqual from 'lodash/isEqual';
import identity from 'lodash/identity';
import {
	defer,
	distinctUntilChanged,
	EMPTY,
	Observable,
	type MonoTypeOperatorFunction,
	type OperatorFunction,
	timestamp,
	pairwise,
	map,
	pipe,
	scan
} from 'rxjs';
import { tap } from 'rxjs';
import { browser } from '$app/environment';

export const labeledLog = <T>(
	name: string,
	fn: (x: T) => unknown = identity
): MonoTypeOperatorFunction<T> =>
	tap({
		error: (e) => console.error(name, e),
		next: (x) => console.log(name, fn(x)),
		complete: () => console.log(name, 'complete')
	});

export const distinctBy = <T, R>(
	keyFn: (t: T) => R = identity,
	checkFn: (ra: R, rb: R) => boolean = isEqual
): MonoTypeOperatorFunction<T> => distinctUntilChanged((a, b) => checkFn(keyFn(a), keyFn(b)));

export const browserOnly = <T>(
	source: Observable<T>,
	fallback: Observable<T> = EMPTY as Observable<T>
): Observable<T> =>
	defer(() => {
		if (!browser) return fallback;

		return source;
	});

const trailing = <T>(n: number): OperatorFunction<T, T[]> =>
	pipe(scan((xs, x) => [...xs, x].slice(-n), [] as T[]));

export const byRate = (samples: number): OperatorFunction<unknown, number> =>
	pipe(
		timestamp(),
		pairwise(),
		map(([a, b]) => b.timestamp - a.timestamp),
		trailing(10),
		// (source) => {
		// 	const os = source.pipe(share());
		// 	const clear = os.pipe(
		// 		debounceTime(1500),
		// 		map(() => [])
		// 	);
		// 	const trail = os.pipe(trailing(samples));
		// },
		map((xs) => (1000 * samples) / xs.reduce((a, b) => a + b, 0))
	);

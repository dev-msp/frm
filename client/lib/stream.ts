import type { MonoTypeOperatorFunction } from 'rxjs';
import { tap, identity } from 'rxjs';

export const labeledLog = <T>(
	name: string,
	fn: (x: T) => unknown = identity
): MonoTypeOperatorFunction<T> =>
	tap({
		error: (e) => console.error(name, e),
		next: (x) => console.log(name, fn(x)),
		complete: () => console.log(name, 'complete')
	});

import { scan, type OperatorFunction, debounceTime, share, map, mergeWith } from 'rxjs';

export const expo =
	({
		min,
		max,
		factor = 2,
		cancel = 1200
	}: {
		min: number;
		max: number;
		factor: number;
		cancel: number;
	}): OperatorFunction<unknown, number> =>
	(source) => {
		const s = source.pipe(
			map(() => true),
			share()
		);

		const reset = s.pipe(
			debounceTime(cancel),
			map(() => false)
		);

		return s.pipe(
			mergeWith(reset),
			scan((xs, x) => (x ? Math.min(xs * factor, max) : min), min)
		);
	};

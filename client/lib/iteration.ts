export const mapObject = <K extends string | symbol, T, R>(
	obj: Record<K, T>,
	mapper: (v: T, k: K) => R
): Record<K, R> => {
	const output = {} as Record<K, R>;
	Object.entries<T>(obj).forEach(([k, v]) => {
		output[k as K] = mapper(v, k as K);
	});
	return output;
};

export const constantize = (keys: string[]): Record<string, string> => {
	const output: Record<string, string> = {};
	keys.forEach((key) => {
		output[key] = key;
	});
	return output;
};

export const range = (lo: number, hi: number, step = 1): number[] => {
	if (typeof hi === 'undefined') {
		hi = lo;
		lo = 0;
	}
	const output = [];
	for (let i = lo; i < hi; i += step) {
		output.push(i);
	}
	return output;
};

export const pick =
	<T, K extends string | number | symbol>(keys: K[]) =>
	(o: Record<K, T>): Record<K, T> => {
		const result = {} as Record<K, T>;

		for (const k of keys) {
			if (k in o) {
				result[k] = o[k];
			}
		}
		return result;
	};

// export const omit = R.curryN(2, (keys, obj) => {
//   const noKeysInKeys = R.filter(pipe(R.nth(0), notContainedIn(keys)));
//   return transduceObject(noKeysInKeys, obj);
// });

export const omit =
	<T, K extends string | number | symbol>(keys: K[]) =>
	(o: Record<K, T>): Record<K, T> => {
		const result = {} as Record<K, T>;

		for (const [k, v] of Object.entries<T>(o)) {
			if (!keys.includes(k as K)) {
				result[k as K] = v;
			}
		}
		return result;
	};

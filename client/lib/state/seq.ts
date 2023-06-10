// Each sequence can potentially be part of a larger one, so we need indexes
// for both the start and end relative to the main sequence.
//
// sequence
// |---------------------------------------------------------------------------|
// s                                                                           e
//
// represented as subsequences
// |------------------|------------------|------------------|------------------|
// 0                  1               ...i...                                  n

export const index = (s: number, e: number, i: number, n: number) =>
	Math.round(s + (e - s) * (i / n));

export const range = (from: number, to: number, steps: number, trailing = false) => {
	if (!steps) return [];
	const minTo = Math.max(to, from + steps);
	const n = steps + Number(!trailing);
	return _range(n).map((_, i: number) => {
		const s = index(from, minTo, i, n - 1);
		return Math.round(s);
	});
};

const { pow } = Math;
const distributions = {
	// normal: (x: number): number => {
	// 	const variance = 1 / sqrt(2 * PI);
	//
	// 	const coefficient = 1 / (sqrt(2 * PI) * variance);
	// 	const exponent = -0.5 * pow((x - 1) / variance, 2);
	//
	// 	return coefficient * Math.exp(exponent);
	// },
	quadratic: (x: number): number => 4 * pow(x - 0.5, 2)
};

const sumsTo = (total: number, dist: number[]): number[] => {
	const distMax = dist.reduce((a, b) => a + b, 0);
	return dist.map((x) => (x * total) / distMax);
};

const _range = (lo: number, hi?: number): number[] => {
	if (typeof hi === 'undefined') {
		hi = lo;
		lo = 0;
	}

	return [...Array(hi - lo)].map((_, i) => lo + i);
};

const allocateFairly = (xs: number[], remaining: number): number[] => {
	if (remaining <= 0 || xs.length === 0) return xs;
	let i = 0;
	const copy = [...xs];
	// eslint-disable-next-line no-constant-condition
	while (remaining !== 0 || i < copy.length) {
		const idx = i % copy.length;
		const floor = (i - idx) / copy.length;
		const toAdd = Math.max(floor, 1);
		if ((remaining > 0 && copy[idx] < toAdd) || copy[idx] === 0) {
			copy[idx] = toAdd ?? 1;
			remaining -= toAdd ?? 1;
		} else if (remaining < 0 && copy[idx] && copy[idx] > 1) {
			copy[idx] -= 1;
			remaining += 1;
		}
		i += 1;
	}
	return copy;
};

export const segments = (n: number, max: number): number[] => {
	const half = Math.ceil(n / 2);
	const r = _range(half).map((i) => distributions.quadratic(i / (n - 1)));
	const template = sumsTo(Math.floor(max / 2), r).map((n) => Math.round(n));

	const output: number[] = Array(n).fill(0);
	let total = 0;
	for (const [i, weight] of template.entries()) {
		output[i] = weight;
		total += weight;
		if (i !== (n - 1) / 2) {
			output[n - 1 - i] = weight;
			total += weight;
		}
	}

	return allocateFairly(output, max - total);
};

export const pairwise = (numbers: number[]): [number, number][] => {
	if (!numbers.length) return [];
	const result = [];
	for (let i = 0; i < numbers.length - 1; i++) {
		result.push([numbers[i], numbers[i + 1]] as [number, number]);
	}
	return result;
};

export const quadraticSqueeze = (start: number, end: number, steps: number): number[] => {
	const sg = segments(steps, steps);
	const r = pairwise(range(start, end, steps));

	return r
		.flatMap(([s, e], i) => pairwise(range(s, e, sg[i])))
		.map(([s, e], i) => index(s, Math.min(e, end), i, steps));
};

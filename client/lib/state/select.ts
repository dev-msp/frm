import { range } from 'd3-array';
import { scaleLinear } from 'd3-scale';
import { easeCubicInOut } from 'd3-ease';
import type { State } from './';
import type { Range } from './range';
import type { Grid } from './grid';
import * as seq from './seq';

export type Selector<T, S = State> = (s: S) => T;

export const useHours: Selector<boolean, Range> = ({ totalDuration }) =>
	totalDuration > 60 * 60 * 1000;

export const bounds: Selector<{ start: number; end: number }, Range> = ({
	from,
	to,
	totalDuration
}) => {
	if (from < 0 || to < 0 || from + to === 0) {
		return { start: 0, end: totalDuration };
	}
	const [start, end] = to >= from ? [from, to] : [to, from];
	return { start, end };
};

export const data: Selector<number[], Grid> = (s) => {
	const { start, end } = bounds(s.range);
	const size = s.dim * s.dim;
	if (s.interpolation === 'linear') {
		// quantize to resolution
		return seq.range(start, end, size, false);
	}
	// quantize to resolution

	const scale = scaleLinear().range([start, end]);
	return range(size).map((t) => scale(easeCubicInOut(t)));
};

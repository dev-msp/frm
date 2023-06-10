import type { Command } from '$lib/command';
import type { Reducer } from './reducer';

export type Range = {
	from: number;
	to: number;
	totalDuration: number;
};

const RANGE_MIN = 1e3;

export const rangeReducer: Reducer<Range, Command> = (s, a) => {
	switch (a.type) {
		case 'clear-input':
			return {
				...s,
				from: RANGE_MIN,
				to: s.totalDuration - 50
			};
		case 'set':
			return {
				...s,
				from: Math.max(RANGE_MIN, a.from ?? s.from),
				to: Math.min(a.to ?? s.to, s.totalDuration - 10)
			};
		case 'shift':
			switch (a.unit) {
				case 'second': {
					const amount = a.amount * 1000;
					return {
						...s,
						from: Math.max(RANGE_MIN, s.from + amount),
						to: Math.min(s.totalDuration, s.to + amount)
					};
				}
				case 'page': {
					const pageSize = s.to - s.from;

					return {
						...s,
						from: Math.max(RANGE_MIN, s.from + pageSize * a.amount),
						to: Math.min(s.totalDuration, s.to + pageSize * a.amount)
					};
				}
				default:
					return s;
			}
		case 'zoom': {
			const amount = a.amount * 1000;
			// TODO make sure reverse isn't possible
			const newSize = s.to - s.from - amount;
			return {
				...s,
				to: Math.min(s.totalDuration, Math.round(s.from + newSize))
			};
		}
		default:
			return s;
	}
};

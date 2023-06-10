import type { Command } from '$lib/command';
import type { Reducer } from './reducer';

export type Range = {
	from: number;
	to: number;
	totalDuration: number;
};

export const rangeReducer: Reducer<Range, Command> = (s, a) => {
	switch (a.type) {
		case 'clear-input':
			return {
				...s,
				from: 0,
				to: s.totalDuration - 50
			};
		case 'set':
			return {
				...s,
				from: Math.max(0, a.from ?? s.from),
				to: Math.min(a.to ?? s.to, s.totalDuration - 10)
			};
		case 'shift':
			switch (a.unit) {
				case 'second': {
					const amount = a.amount * 1000;
					return {
						...s,
						from: Math.max(0, s.from + amount),
						to: Math.min(s.totalDuration, s.to + amount)
					};
				}
				case 'page': {
					const pageSize = s.to - s.from;

					return {
						...s,
						from: Math.max(0, s.from + pageSize * a.amount),
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

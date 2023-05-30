import type { Command } from '$lib/command';

export type Selection =
	| { type: 'none' }
	| {
			type: 'single';
			value: string;
	  }
	| {
			type: 'double';
			value: [string, string];
	  };

export type State = {
	from: number;
	to: number;
	totalDuration: number;
	selection: Selection;
	data: [number, number][];
};

export const initial: Omit<State, 'totalDuration'> = {
	from: 0,
	to: -1,
	selection: { type: 'none' } as Selection,
	data: []
};

export const selectionReducer = (s: Selection, a: Command): Selection => {
	switch (a.type) {
		case 'clear-input':
		case 'shift':
		case 'zoom':
		case 'data':
		case 'set':
			return { type: 'none' };
		case 'alpha': {
			switch (s.type) {
				case 'none':
					return { type: 'single', value: a.key };
				case 'single':
					return { type: 'double', value: [s.value, a.key] };
				case 'double':
					return { type: 'double', value: [s.value[1], a.key] };
				default:
					return s;
			}
		}
		default:
			return s;
	}
};

export const reducer = (s: State, a: Command) => {
	const state = { ...s, selection: selectionReducer(s.selection, a) };
	switch (a.type) {
		case 'clear-input':
			return {
				...s,
				selection: { type: 'none' } as Selection,
				from: 0,
				to: s.totalDuration
			};
		case 'set':
			return {
				...s,
				from: a.from ?? s.from,
				to: a.to ?? s.to
			};
		case 'shift':
			return {
				...state,
				from: Math.max(0, s.from + a.amount),
				to: Math.min(s.totalDuration, s.to + a.amount)
			};
		case 'zoom': {
			// TODO make sure reverse isn't possible
			const halfSpan = (s.to - s.from) / 2;
			const center = s.from + halfSpan;
			const fromB = center - halfSpan * (1 - a.amount);
			const toB = center + halfSpan * (1 - a.amount);
			return {
				...state,
				from: Math.max(0, Math.floor(fromB)),
				to: Math.min(s.totalDuration, Math.floor(toB))
			};
		}
		case 'unknown_keypress':
			return state;
		case 'data':
			return { ...state, data: a.payload };
		default:
			return state;
	}
};

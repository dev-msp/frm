import type { Command } from '$lib/command';
import { rangeReducer, type Range } from './range';
import type { Reducer } from './reducer';
import type { Selector } from './select';

export type Selection =
	| { type: 'none' }
	| { type: 'single'; value: string }
	| { type: 'double'; value: [string, string] };

export type Grid = {
	dim: number;
	range: Range;
	selection: Selection;
	resolution: number;
	interpolation: 'linear' | 'quadratic';
};

export const selectionReducer: Reducer<Selection, Command> = (s, a) => {
	switch (a.type) {
		case 'clear-input':
		case 'toggleInterpolation':
		case 'shift':
		case 'zoom':
		case 'grid':
		case 'set':
			return { type: 'none' };
		case 'key': {
			switch (s.type) {
				case 'none': {
					console.log('none');
					return { type: 'single', value: a.key };
				}
				case 'single': {
					console.log('single');
					return { type: 'double', value: [s.value, a.key] };
				}
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

export const highlightedChars: Selector<string[], Selection> = (sel) => {
	switch (sel.type) {
		case 'none':
			return [];
		case 'single':
			return [sel.value];
		case 'double':
			return sel.value;
	}
};

export const initGrid = (duration: number): Grid => ({
	range: { from: 1e3, to: duration, totalDuration: duration },
	selection: { type: 'none' },
	resolution: 1e3,
	dim: 4,
	interpolation: 'linear'
});

export const gridReducer: Reducer<Grid, Command> = (s, a) => {
	console.log('called gridReducer with', JSON.stringify({ s, a }, null, 2));
	const state = {
		...s,
		selection: selectionReducer(s.selection, a),
		range: rangeReducer(s.range, a)
	};
	switch (a.type) {
		case 'toggleInterpolation':
			return {
				...state,
				interpolation: s.interpolation === 'linear' ? 'quadratic' : 'linear'
			};
		case 'grid':
			return { ...state, dim: state.dim + a.amount };
		default:
			return state;
	}
};

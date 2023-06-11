import type { Command } from '$lib/command';
import { rangeReducer, type Range } from './range';
import type { Reducer } from './reducer';
import type { Selector } from './select';

export type Selection =
	| { type: 'none' }
	| { type: 'single'; value: string }
	| { type: 'double'; value: [string, string] };

type InputMode = { mode: 'select-range'; selection: Selection } | { mode: 'default' };

export type Grid = {
	dim: number;
	range: Range;
	input: InputMode;
	resolution: number;
	interpolation: 'linear' | 'quadratic';
};

export const inputModeReducer: Reducer<InputMode, Command> = (s, a) => {
	if ((['set', 'clear-input', 'accept-context'] as Command['type'][]).includes(a.type)) {
		return { mode: 'default' };
	}
	switch (s.mode) {
		case 'default':
			return defaultModeReducer(s, a);
		case 'select-range':
			return { ...s, selection: selectionReducer(s.selection, a) };
	}
};

const defaultModeReducer: Reducer<InputMode, Command> = (s, a) => {
	if (a.type === 'key' && a.key === 's') {
		return { mode: 'select-range', selection: { type: 'none' } };
	}
	return s;
};

export const selectionReducer: Reducer<Selection, Command> = (s, a) => {
	switch (a.type) {
		case 'clear-input':
		case 'shift':
		case 'zoom':
		case 'grid':
		case 'set':
			return { type: 'none' };
		case 'key': {
			switch (s.type) {
				case 'none': {
					return { type: 'single', value: a.key };
				}
				case 'single': {
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
	input: { mode: 'default' },
	resolution: 1e3,
	dim: 3,
	interpolation: 'linear'
});

export const gridReducer: Reducer<Grid, Command> = (s, a) => {
	const state = {
		...s,
		input: inputModeReducer(s.input, a),
		range: rangeReducer(s.range, a)
	};
	switch (a.type) {
		case 'grid':
			return { ...state, dim: state.dim + a.amount };
		default:
			return state;
	}
};

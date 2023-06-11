import type { Command } from '$lib/command';
import { gridReducer, type Grid, initGrid } from './grid';
import { reducer as bisectReducer, initial as initBisect, type Bisect } from './bisect';
import type { PartReducer, Reducer } from './reducer';
import type { Selector } from './select';
import { identity } from '$lib/function';

export type Selection =
	| { type: 'none' }
	| { type: 'single'; value: string }
	| { type: 'double'; value: [string, string] };

export type State = {
	view: View;
};

export type View = { type: 'loading' } | ({ type: 'grid' } & Grid) | ({ type: 'bisect' } & Bisect);

export const initial: State = {
	view: { type: 'loading' }
};

export const selectionReducer = (s: Selection, a: Command): Selection => {
	switch (a.type) {
		case 'clear-input':
		case 'shift':
		case 'zoom':
		case 'grid':
		case 'set':
			return { type: 'none' };
		case 'key': {
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

type ReducerMap = {
	[K in View['type']]: PartReducer<Omit<View & { type: K }, 'type'>, Command, State>;
};

const viewReducers: ReducerMap = {
	loading: identity,
	grid: gridReducer,
	bisect: bisectReducer
};

const nextView: PartReducer<View, Command, State> = (s, a, f) => {
	switch (a.type) {
		case 'accept-context': {
			if (s.type === 'loading') {
				return { type: 'grid', ...initGrid(a.totalDuration) };
			} else if (s.type === 'grid') {
				return { ...s, range: { ...s.range, totalDuration: a.totalDuration } };
			} else if (s.type === 'bisect') {
				return {
					...s,
					original: { ...s.original, totalDuration: a.totalDuration },
					current: { ...s.current, totalDuration: a.totalDuration }
				};
			}
			return s;
		}
		case 'reset': {
			if (s.type === 'grid') {
				return { type: 'grid', ...initGrid(s.range.totalDuration) };
			} else if (s.type === 'bisect') {
				return { type: 'bisect', ...initBisect(s.current.totalDuration) };
			}
			return s;
		}
		default: {
			if (s.type === 'loading') {
				return { type: 'loading' };
			} else if (s.type === 'bisect') {
				return { ...s, ...viewReducers.bisect(s, a, f) };
			} else if (s.type === 'grid') {
				return { ...s, ...viewReducers.grid(s, a, f) };
			}
			return s;
		}
	}
};

export const reducer: Reducer<State, Command> = (s, a) => {
	return { view: nextView(s.view, a, s) };
};

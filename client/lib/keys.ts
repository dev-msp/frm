import {
	EMPTY,
	Observable,
	Subject,
	concatMap,
	debounceTime,
	distinctUntilChanged,
	filter,
	fromEvent,
	identity,
	map,
	of,
	scan,
	share,
	shareReplay,
	startWith,
	switchMap,
	tap,
	type MonoTypeOperatorFunction
} from 'rxjs';
import { fromFetch } from 'rxjs/fetch';
import { getRange } from './api';

export const alpha = 'abcdefghijklmnopqrstuvwxyz';

// if (e.key === 'Escape') {
//   // reset();
//   // save();
//   return;
// } else if (e.key === '{') {
//   // shift(-1, -1);
// } else if (e.key === '}') {
//   // shift(1, 1);
// } else if (e.key === '[') {
//   // shift(-5, -5);
// } else if (e.key === ']') {
//   // shift(5, 5);
// } else if (e.key === '-') {
//   // shift(-30, 30);
// } else if (e.key === '=') {
//   // shift(30, -30);
// }
// const match = alpha.indexOf(e.key);
// if (match < 0) {
//   return;
// }
// if (!fromTmpl && !toTmpl) {
//   fromTmpl = result?.codes[match][0];
// } else {
//   if (!toTmpl) {
//     toTmpl = result?.codes[match][1];
//   }
//   save();
// }

type UnknownCommand = {
	type: 'unknown_keypress';
	key: string;
};

type TraversalCommand =
	| { type: 'zoom'; amount: number }
	| { type: 'shift'; both: number }
	| { type: 'shift'; from?: number; to?: number }
	| { type: 'set'; from?: number; to?: number; n?: number };

type UiStateCommand = {
	type: 'clear-input';
};

type AlphaCommand = {
	type: 'alpha';
	key: string;
};

type DataEvent = {
	type: 'data';
	payload: [number, number][];
};

type Command = UiStateCommand | AlphaCommand | TraversalCommand | DataEvent | UnknownCommand;

const commandMapping: Partial<Record<string, Command>> = {
	'(': { type: 'shift', both: -5 },
	')': { type: 'shift', both: 5 },
	'[': { type: 'shift', both: -5 },
	']': { type: 'shift', both: 5 },
	'{': { type: 'shift', both: -1 },
	'}': { type: 'shift', both: 1 },
	'-': { type: 'zoom', both: 0.3 },
	'=': { type: 'zoom', amount: 0.3 },
	Escape: { type: 'clear-input' }
};

type Selection =
	| { type: 'none' }
	| {
			type: 'single';
			value: string;
	  }
	| {
			type: 'double';
			value: [string, string];
	  };

type State = {
	from: number;
	to: number;
	totalDuration: number;
	selection: Selection;
	data: [number, number][];
};

const eventToCommand = ({ key }: { key: string }): Command =>
	commandMapping[key] ??
	(/^[a-z]$/i.test(key) ? { type: 'alpha', key } : undefined) ?? {
		type: 'unknown_keypress',
		key
	};

const loadContext: Observable<{ duration: number }> = fromFetch('/api/context', {
	method: 'GET',
	headers: { accept: 'application/json' }
}).pipe(concatMap((res) => res.json()));

export const commands = fromEvent<KeyboardEvent>(document, 'keydown').pipe(
	filter(({ repeat }) => !repeat),
	map(eventToCommand),
	share()
);

export const alphas = commands.pipe(filter((c): c is AlphaCommand => c.type === 'alpha'));
export const simpleCommands = commands.pipe(
	filter((c): c is TraversalCommand => c.type === 'zoom')
);

export const initial: Omit<State, 'totalDuration'> = {
	from: 0,
	to: -1,
	selection: { type: 'none' } as Selection,
	data: []
};

const selectionReducer = (s: Selection, a: Command): Selection => {
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

const reducer = (s: State, a: Command) => {
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

const allCommands = new Subject<Command>();
commands.subscribe(allCommands);

export const state = loadContext.pipe(
	concatMap(({ duration }) => {
		const initialWithDuration = { ...initial, to: duration, totalDuration: duration };

		return allCommands.pipe(
			scan(reducer, initialWithDuration as State),
			labeledLog('state', (x) => JSON.stringify({ ...x, data: 'data' }, null, 2)),
			startWith(initialWithDuration as State)
		);
	}),
	shareReplay(1)
);

const resequences: Observable<TraversalCommand> = state.pipe(
	distinctUntilChanged((a, b) => a.selection.type === b.selection.type),
	filter((s) => s.selection.type === 'double'),
	concatMap(({ selection, data }) => {
		if (selection.type !== 'double') return EMPTY;

		const [i, j] = selection.value.map((c) => alpha.indexOf(c));
		if (i < 0 || j < 0) {
			throw new Error('should be impossible');
		}

		return of({
			type: 'set',
			from: Math.round(data[i][0]),
			to: Math.round(data[j][1])
		} as TraversalCommand);
	})
);
resequences.subscribe(allCommands);

const labeledLog = <T>(
	name: string,
	fn: (x: T) => unknown = identity
): MonoTypeOperatorFunction<T> =>
	tap({
		error: (e) => console.error(name, e),
		next: (x) => console.log(name, fn(x)),
		complete: () => console.log(name, 'complete')
	});

export const data: Observable<DataEvent> = state.pipe(
	map((s) => [s.from, s.to, 20]),
	distinctUntilChanged((a, b) => a.every((n, i) => n === b[i])),
	debounceTime(300),
	switchMap(([from, to, n]) => getRange({ from, to }, n)),
	startWith({ codes: [], start: '', end: '' }),
	map(({ codes: payload }) => ({ type: 'data', payload } as DataEvent))
);

data.subscribe(allCommands);

import {
	EMPTY,
	Observable,
	Subject,
	concatMap,
	debounceTime,
	distinctUntilChanged,
	filter,
	map,
	of,
	pipe,
	scan,
	shareReplay,
	startWith,
	switchMap,
	type OperatorFunction
} from 'rxjs';
import { fromFetch } from 'rxjs/fetch';
import { getRange } from './api';
import { alpha, commands, type Command, type DataEvent, type TraversalCommand } from './command';
import type { State } from './state';
import { initial, reducer } from './state';
import { labeledLog } from './stream';

const loadContext: Observable<{ duration: number }> = fromFetch('/api/context', {
	method: 'GET',
	headers: { accept: 'application/json' }
}).pipe(concatMap((res) => res.json()));

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

const resequences: OperatorFunction<State, TraversalCommand> = pipe(
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

resequences(state).subscribe(allCommands);

const data: OperatorFunction<State, DataEvent> = pipe(
	map((s) => [s.from, s.to, 25]),
	distinctUntilChanged((a, b) => a.every((n, i) => n === b[i])),
	debounceTime(300),
	switchMap(([from, to, n]) => getRange({ from, to }, n)),
	startWith({ codes: [], start: '', end: '' }),
	map(({ codes: payload }) => ({ type: 'data', payload } as DataEvent))
);

data(state).subscribe(allCommands);

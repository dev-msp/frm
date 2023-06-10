import {
	Observable,
	concatMap,
	debounceTime,
	exhaustMap,
	filter,
	map,
	of,
	shareReplay,
	startWith
} from 'rxjs';
import { fromFetch } from 'rxjs/fetch';
import {
	alpha,
	commands,
	type Command,
	type TraversalCommand,
	type UiStateCommand
} from './command';
import { initial, type State, type View } from './state';
import { reducer } from './state';
import { Store } from './state/store';
import { combineEpics, type AppEpic, type Epic } from './state/stream';
import * as select from './state/select';
import * as seq from './state/seq';
import { distinctBy, browserOnly, labeledLog } from './stream';
import type { Grid } from './state/grid';

const loadContext: Observable<{ duration: number }> = browserOnly(
	fromFetch('/api/context', {
		method: 'GET',
		headers: { accept: 'application/json' }
	})
).pipe(concatMap((res) => res.json()));

export const store = new Store<State, Command>(reducer, initial);

export const state$ = store.runEpic((_a, s) => s.pipe(distinctBy()), store.getState(), {
	share: true
});

function sortStringWithNumbers(input: string[]): string[] {
	const letters = [];
	const numbers = [];

	for (let i = 0; i < input.length; i++) {
		if (isNaN(parseInt(input[i]))) {
			letters.push(input[i]);
		} else {
			numbers.push(input[i]);
		}
	}

	letters.sort();
	numbers.sort(function (a, b) {
		return parseInt(a) - parseInt(b);
	});

	return [...letters, ...numbers];
}

export const resequences: AppEpic<TraversalCommand> = (cmds, state) => {
	return state.pipe(
		map((s) => s.view),
		filter((s): s is View & { type: 'grid' } => s.type === 'grid'),
		distinctBy((s) => s.selection.type),
		filter(
			(s): s is View & { type: 'grid'; selection: { type: 'double' } } =>
				s.selection.type === 'double'
		),
		concatMap((state) => {
			const { selection, dim, range } = state;

			const data = seq
				.pairwise(select.data(state))
				.map(([a, b], i, arr) => [a, seq.index(a, b, i, arr.length - 1), b]);

			let [i, j] = sortStringWithNumbers(selection.value).map((c) => alpha.indexOf(c));

			if (i < 0 || j < 0) {
				throw new Error('should be impossible');
			}

			if (i === j) {
				j = Math.min(j + 1, dim * dim - 1);
				i = Math.max(0, j - 1);
			}

			return of({
				type: 'set',
				from: Math.round(data[i][0]),
				to: Math.min(Math.round(data[j][1]), range.totalDuration)
			} as TraversalCommand);
		})
	);
};

const dataDeps: select.Selector<[number, number, number, Grid['interpolation']], Grid> = (s) => [
	s.range.from,
	s.range.to,
	Math.min(s.dim * s.dim, 36),
	s.interpolation
];

export const data: Epic<Grid, Command, number[]> = (action, state) =>
	state.pipe(
		distinctBy(dataDeps),
		debounceTime(250),
		map(select.data),
		startWith([]),
		shareReplay(1)
	);

const rootEpic = combineEpics(commands, resequences, (cmds) => {
	const loadIt = loadContext.pipe(
		map(
			({ duration }) =>
				({ type: 'accept-context', totalDuration: duration - 1000 } as UiStateCommand)
		)
	);
	return cmds.pipe(
		filter(({ type }) => type === 'reset'),
		startWith(undefined),
		exhaustMap(() => loadIt)
	);
});

store.consumeEpic(rootEpic);

// State changes, frames are rendered with their timecode as a prop.

// 100 200 300
// 400 500 600
// 700 800 900
//
// "100 400"
//
// 100 ___ ___
// ___ ___ ___  What fills in here? Well, what do we got?
// ___ ___ 400
//
// 100 138 175
// 213 250 288  Normally we'd just do this
// 325 363 400
//
//
// 100 ___ ___
// 200 ___ 300  We know we already loaded 200, 300 and 400.
// ___ ___ 400  Could we just load them in the places they're the closest to?
//
// 100 100 200
// 200 300 300  Then fill forward (or something)
// 300 400 400

// const allFrames = new Array(totalDuration);

// function fillWithCachedFrames(from: number, to: number, frames: (number | undefined)[]): number[] {
// 	const filledFrames: number[] = [];
//
// 	// Loop through each frame.
// 	for (let i = 0; i < frames.length; i++) {
// 		// If the frame is undefined, look for the nearest previous loaded frame.
// 		if (frames[i] === undefined) {
// 			// Loop backwards from the current frame checking for a loaded frame.
// 			for (let j = i; j >= 0; j--) {
// 				const f = frames[j];
// 				if (f !== undefined) {
// 					// Fill with the previous loaded frame and break out of the loop.
// 					filledFrames.push(f);
// 					break;
// 				}
// 			}
// 		} else {
// 			// If the frame is not undefined, add it to the filled frames.
// 			filledFrames.push(frames[i]);
// 		}
// 	}
//
// 	return filledFrames;
// }

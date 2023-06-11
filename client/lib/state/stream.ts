import { Observable, concatAll, filter, from, map, merge, mergeAll } from 'rxjs';

import type { Command } from '$lib/command';
import type { State } from './';

export type Epic<S, A, T> = (commands: Observable<A>, state: Observable<S>) => Observable<T>;
export type AppEpic<T> = Epic<State, Command, T>;
export type AnonymousEpic<T> = Epic<unknown, unknown, T>;

export type RunEpicConfig = {
	share?: boolean;
	replayCount?: number;
	label?: string;
};

export const runEpicDefault: RunEpicConfig = {
	share: false,
	replayCount: 1
};

export type EpicTransform<S, A, T, Sx = S, Ax = A, Tx = T> = (
	input: Epic<S, A, T>
) => Epic<Sx, Ax, Tx>;

export const mapEpicState = <S, A, T, Sx>(
	fn: (s: S) => Sx | undefined
): EpicTransform<Sx, A, T, S> => {
	return (epic) => (action$, state$) =>
		epic(
			action$,
			state$.pipe(
				map(fn),
				filter((x): x is Sx => typeof x !== 'undefined')
			)
		);
};

export const combineEpics =
	(...epics: AppEpic<Command>[]): Epic<State, Command, Command> =>
	(actions$, state$) =>
		merge(epics.map((e) => e(actions$, state$))).pipe(mergeAll());

export const sequenceEpics =
	<S, A, T>(...epics: Epic<S, A, T>[]): Epic<S, A, T> =>
	(actions$, state$) =>
		from(epics.map((e) => e(actions$, state$))).pipe(concatAll());
